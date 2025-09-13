<script setup lang="ts">
// 使用 Tauri API npm 包时:
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

// 使用 Tauri 全局脚本（不使用 npm 包时）
// 确保在 `tauri.conf.json` 中设置 `app.withGlobalTauri` 为 true
// const invoke = window.__TAURI__.core.invoke;

const message = ref("");



// 调用命令
// invoke('my_custom_command', {invokeMessage: 'Kotlin' }).then((msg) => message.value = msg as string);
const number = ref("");

function onClick() {
    const n = parseInt(number.value);
    if(isNaN(n)) {
        window.alert("请输入数字");
        return;
    }
    invoke('greet', { num:  n }).then((msg) => {
        window.alert(msg);
    }) .catch((err) => {
        window.alert(err);
    })
}


</script>

<template>
 <div>
    <h1>猜数字</h1>
    <div>
        <input type="text" placeholder="请输入你要猜的数字" v-model="number">
        <button @click="onClick" >提交</button>
    </div>
 </div>
</template>

<style scoped>

</style>
