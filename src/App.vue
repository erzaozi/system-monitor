<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { onMounted, onUnmounted, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let trayInterval: NodeJS.Timeout | null = null;

// 获取整个 body 的高度
function getBodyHeight() {
    const html = document.documentElement;
  const rect = html.getBoundingClientRect();
  console.log(rect)

    console.log("Body Height: ", rect.height); // 打印高度方便调试
    return rect.height
}

// 调整窗口高度
async function adjustWindowHeight() {
    const height = getBodyHeight();
    console.log("Adjusting window height to:", height);
    if (height > 0) {
        await invoke("resize_window", { height });
    }
}
const flushTray = () => {
    invoke<boolean>("update_tray_title", {});
};

onMounted(async () => {
    adjustWindowHeight();
    window.addEventListener("resize", adjustWindowHeight);
    trayInterval = setInterval(flushTray, 5000);
});

onUnmounted(() => {
    if (trayInterval) {
        clearInterval(trayInterval); // 清除定时器
    }
    window.removeEventListener("resize", adjustWindowHeight);
});

</script>

<template>
    <el-container class="unselectable">
        <el-main ref="contentRef">
            <Greet />
        </el-main>
    </el-container>
</template>

<style scoped>
.el-main {
    padding: 8px;
}
</style>
