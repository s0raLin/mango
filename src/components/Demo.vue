<template>
<input type="text" placeholder="请输入指令" v-model="command" />
<input type="text" placeholder="请输入参数" v-model="args" />
<button @click="onClick" >执行</button>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

let command = ref("");
let args = ref("");

function onClick() {
  invoke('term', {command: command.value, args: args.value})
  .then((msg) => {
    window.alert(msg);
  })
  .catch((err) => {
    window.alert(err);
  })
}
</script>


<style scoped>

</style>