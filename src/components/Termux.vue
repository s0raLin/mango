<template>
  <div class="terminal" @click="focusInput">
    <div class="terminal-output">
      <div v-for="(line, i) in output" :key="i" class="terminal-line">
        <span v-if="line.prompt" class="prompt">{{ line.prompt }}</span>
        <span class="text" v-html="line.htmlText || line.text"></span>
      </div>
    </div>
    <div class="terminal-input">
      <span class="prompt">{{ prompt }}</span>
      <input
        ref="inputRef"
        v-model="currentInput"
        @keydown="handleKeyDown"
        class="input-field"
        spellcheck="false"
        autofocus
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import Convert from 'ansi-to-html';

const convert = new Convert();

function ansiToHtml(text: string): string {
  return convert.toHtml(text)
}

interface TerminalLine {
  prompt: string;
  text: string;
  htmlText?: string;
}

const shell = ref("bash");
const prompt = ref(`[${shell.value}]$ `); //终端提示符

const output = ref<TerminalLine[]>([]);
const currentInput = ref<string>("");
const history = ref<string[]>([]);
let historyIndex = -1;

const inputRef = ref<HTMLInputElement | null>(null);

function focusInput() {
  inputRef.value?.focus();
}

function exitTerminal() {
    invoke('exit');
}

async function handleKeyDown(e: KeyboardEvent) {
    // === 内建命令 ===
    if (currentInput.value === "exit") {
        output.value.push({ prompt: "", text: "退出终端。" });
        currentInput.value = "";
        inputRef.value!.disabled = true; // 禁用输入
        exitTerminal();
        return;
        
    }

    if (currentInput.value === "clear") {
        output.value = [];
        currentInput.value = "";
        return;
    }
  
  if (e.key === "Enter") {
    if (currentInput.value.trim() !== "") {
      // 输入命令
      output.value.push({
        prompt: prompt.value,
        text: currentInput.value,
      });

      try {
         // 拆分命令和参数
        const parts = currentInput.value.trim().split(/\s+/); // 按空格切分
        const command = parts[0]; // 第一个是命令
        const args = parts.slice(1); // 剩下的是参数数组

        // 执行命令，等待结果
        const msg = await invoke<string>("term", {
          command,
          args,
        });
        // 把 ANSI 转义码转换为 HTML
        let html = ansiToHtml(msg)
        output.value.push({
          prompt: "",
          text: msg,
          htmlText: html,
        });
      } catch (err: any) {
        output.value.push({
          prompt: "",
          text: String(err),
        });
      }

      history.value.push(currentInput.value);
      historyIndex = history.value.length;
      currentInput.value = "";
    } else {
        // 输入为空是生成空行
        output.value.push({ prompt: prompt.value, text: "" });
    }
    e.preventDefault();
  }

  // ↑ 历史命令
  if (e.key === "ArrowUp") {
    if (historyIndex > 0) {
      historyIndex--;
      currentInput.value = history.value[historyIndex];
    }
    e.preventDefault();
  }

  // ↓ 历史命令
  if (e.key === "ArrowDown") {
    if (historyIndex < history.value.length - 1) {
      historyIndex++;
      currentInput.value = history.value[historyIndex];
    } else {
      historyIndex = history.value.length;
      currentInput.value = "";
    }
    e.preventDefault();
  }

  // Ctrl + L 清屏
  if (e.ctrlKey && e.key.toLowerCase() === "l") {
    output.value = [];
    currentInput.value = "";
    e.preventDefault();
  }

  // Ctrl + D 退出
  if (e.ctrlKey && e.key.toLowerCase() === "d") {
    // output.value = [];
    // currentInput.value = "";
    // e.preventDefault();

    exitTerminal();
  }
}
</script>

<style scoped>
.terminal {
  background: #1e1e1e;
  color: #d4d4d4;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  padding: 10px;
  border-radius: 6px;
  height: 400px;
  overflow-y: auto;
  cursor: text;
  line-height: 1.2;
}

.terminal-output {
  white-space: pre-wrap;
  margin-bottom: 5px;
  font-variant-ligatures: none;
}

.terminal-line {
  display: flex;
  flex-wrap: nowrap;
  align-items: flex-start;
}

.prompt {
  color: #4ec9b0;
  margin-right: 5px;
  flex-shrink: 0;
}

.terminal-input {
  display: flex;
  align-items: center;
}

.input-field {
  background: transparent;
  border: none;
  outline: none;
  color: #d4d4d4;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  flex: 1;
  line-height: 1.2;
}

.text {
  white-space: pre-wrap;
  word-break: break-all;
  flex: 1;
}
</style>
