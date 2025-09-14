<template>
<div ref="terminalContainer" class="term"></div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted, onBeforeUnmount, computed } from 'vue'
// 导入 xterm 核心库和 FitAddon
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
// 引入 xterm 样式，否则终端不会显示正确
import 'xterm/css/xterm.css'

// 用 ref 获取 DOM 容器
const terminalContainer = ref(null)

// 全局变量，存储终端实例和 fit 插件实例
let term = null
let fitAddon = null


// 输入缓冲区
const commandBuffer = ref('')

// 拆分命令和参数
const cmd = computed(() => commandBuffer.value.split(' ')[0] || '')
const args = computed(() => commandBuffer.value.split(' ').slice(1))
const commandHistory = ref([]) // 命令历史
const historyIndex = ref(-1) // 跟踪历史记录中的当前位置

const currentInput = ref('') // 存储当前输入，用于在历史记录中导航时保存用户输入

// 在组件顶部添加常量
// 定义提示符文本
const PROMPT_TEXT = ref('[bash]$')

// 创建一个计算提示符长度的函数
function getPromptLength() {
  return PROMPT_TEXT.value.length
}

// 组件挂载后执行
onMounted(() => {
  // 创建 xterm 实例，配置终端参数
  term = new Terminal({
    cursorBlink: true,  // 光标闪烁
    rows: 20,           // 默认行数
    cols: 80,           // 默认列数
    theme: {            // 主题颜色
      background: '#1e1e1e',
      foreground: '#ffffff'
    }
  })

  // 创建 fit 插件实例，用于自动调整终端大小
  fitAddon = new FitAddon()
  term.loadAddon(fitAddon)

  // 将终端挂载到容器中
  term.open(terminalContainer.value)

  // 自动调整大小以填满容器
  fitAddon.fit()

  // 写入欢迎信息
  term.writeln('Welcome to xterm.js in Vue!')

  // 定义一个简单的 prompt 方法
  term.prompt = () => {
    term.write(`\r\n${PROMPT_TEXT.value} `) // \r\n 表示换行
  }

  // 显示初始提示符
  term.prompt()

  // 监听用户按键
  term.onKey(e => {
    
    const ev = e.domEvent
    // 判断是否为可打印字符
    const printable = !ev.altKey && !ev.ctrlKey && !ev.metaKey

    const promptLength = getPromptLength() // 动态获取提示符长度

    switch (ev.key) {
        case 'Enter': {
            if(commandBuffer.value.length === 0) {
                term.prompt()
            }
            // 处理命令执行
            handleCommand(commandBuffer.value)
            // 回车时显示新的提示符
            commandHistory.value.push(commandBuffer.value)
            commandBuffer.value = ''
        } break
      
        case 'Backspace': {
            // 只有当光标位置在提示符之后时才允许退格
    const promptLength = getPromptLength() // 动态获取提示符长度
            if (term._core.buffer.x > promptLength+1) { // "[bash]$ " 是 8 个字符
                term.write('\b \b')
                commandBuffer.value = commandBuffer.value.slice(0, -1)
            }
        } break
        case 'ArrowUp': {
            ev.preventDefault()
            if (commandHistory.value.length > 0) {
                if (historyIndex.value === -1) {
                    // 保存当前输入到 currentInput 而不是覆盖 commandHistory
                    currentInput.value = commandBuffer.value
                }
        
                if (historyIndex.value < commandHistory.value.length - 1) {
                    historyIndex.value++
                    const historyCommand = commandHistory.value[commandHistory.value.length - 1 - historyIndex.value]
        
                    // 清除当前行
                    while (commandBuffer.value.length > 0) {
                        if (term._core.buffer.x > 2) {
                            term.write('\b \b')
                            commandBuffer.value = commandBuffer.value.slice(0, -1)
                        }
                    }
        
                    // 显示历史命令
                    term.write(historyCommand)
                    commandBuffer.value = historyCommand
                }
            }
        } break
      
        case 'ArrowDown': {// 下箭头键 
            ev.preventDefault()
            if (historyIndex.value >= 0) {
                historyIndex.value--
        
                // 清除当前行
                while (commandBuffer.value.length > 0) {
                    if (term._core.buffer.x > promptLength) {
                        term.write('\b \b')
                        commandBuffer.value = commandBuffer.value.slice(0, -1)
                    }
                }
        
                if (historyIndex.value === -1) {
                    // 回到原始输入
                    term.write(commandHistory || '')
                    commandBuffer.value = commandHistory || ''
                } else {
                    const historyCommand = commandHistory.value[commandHistory.value.length - 1 - historyIndex.value]
                    term.write(historyCommand)
                    commandBuffer.value = historyCommand
                }
            } 
        } break
        default: {
            if (printable) {
                // 确保光标不会移动到提示符之前（位置8是提示符结束的位置）
                if (term._core.buffer.x >= promptLength) {
                    term.write(e.key)
                    commandBuffer.value += e.key
                }
            }
        } break

    }

    // 处理Ctrl组合键
    if (ev.ctrlKey) {
        switch(ev.key) {
            case 'c': {
                if (commandBuffer.value.length > 0) {
                    // 如果有输入内容，复制内容
                    term.write('^C')
                    commandBuffer.value = ''
                } else {
                    // 如果没有输入内容，模拟中断信号
                    term.write('\r\n^C')
                    term.prompt()
                }
                
            } break
            case 'l': {
                // 清屏命令
                term.write('\x1B[2J\x1B[H')
                term.prompt()
            } break
            case 'd': {
                // 退出命令
                // term.write('[Process exited with code 0]')
                invoke('exit')
                // term.dispose()
            } break
        }
    }
  })
  window.addEventListener('resize', () => {
    fitAddon.fit()
  })
})


// 处理终端命令
function handleCommand(command) {
  command = command.trim()
  
  if (!command) return
  
  switch (command) {
    case 'clear':
      // 清屏命令
      term.write('\x1B[2J\x1B[H') // ANSI 清屏序列
      term.prompt()
      break
    case 'exit':
      // 退出命令
      term.write('[Process exited with code 0]')
      term.dispose()
      invoke('exit') // 退出程序
      break
    case 'history':
      // 显示命令历史, 索引从1开始
      term.write('\r\n' + commandHistory.value.map((cmd, index) => `${index + 1}  ${cmd}`).join('\r\n') + '\r\n')
      break
    default:
         // 特殊处理 fastfetch 命令  
      if (cmd.value === 'fastfetch') {
        invoke('term', { command: cmd.value, args: args.value })
          .then((msg) => {
          // fastfetch 输出可能包含 ANSI 转义序列，直接输出
          term.write('\r\n' + msg)
        })
        .catch((err) => {
          let out = err.toString().trim()
          out = out.replace(/\n/g, '\r\n')
          term.write('\r\n' + out)
        })
        .finally(() => {
          term.prompt()
        })
      }
     // 对于其他命令，调用tauri后端
      invoke('term', { command: cmd.value, args: args.value })
      .then((msg) => {
        let out = msg.trim()
        // 将 \n 换行符转换为终端可识别的 \r\n 格式
        out = out.replace(/\n/g, '\r\n')
        term.write('\r\n' + out)
      })
      .catch((err) => {
        // 这里也有个错误，应该是 err 而不是 msg
        let out = err.toString().trim()
        out = out.replace(/\n/g, '\r\n')
        term.write('\r\n' + out)
      })
      .finally(() => {
        term.prompt()
      })
  }
}

// 组件卸载时释放资源
onBeforeUnmount(() => {
  if (term) {
    term.dispose() // 销毁终端实例
  }
})
</script>

<style scoped>
.term {
  height: 100vh; /* 直接使用视口高度 */
  width: 100vw;  /* 直接使用视口宽度 */
}
</style>
