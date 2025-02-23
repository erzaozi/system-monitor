<script setup lang="ts">
import { ref, onMounted, provide, onUnmounted, VNode, h } from "vue";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";
import 'echarts';
import 'echarts/theme/blue'
import VChart, { THEME_KEY } from 'vue-echarts';
import { NetworkData } from "../assets/ts/monitor";
import { setNetworkGuage, networkOption } from "../assets/ts/options/networkOption";
import { EChartsType } from "echarts/core";
provide(THEME_KEY, 'blue')

const networkUsageCharts = ref<EChartsType>();
let unlisten: UnlistenFn | null = null;

onMounted(async () => {
    networkUsageCharts.value?.setOption(networkOption);
    try {
        unlisten = await listen("network_info", (event: Event<NetworkData>) => {
            if (networkUsageCharts.value) {
                setNetworkGuage(networkUsageCharts.value, event.payload);
            }
        });
    } catch (error) {
        console.error("Failed to register event listener:", error);
    }
})

onUnmounted(() => {
    if (unlisten) {
        unlisten();
        unlisten = null;
    }
})


</script>

<template>
    <el-card>
        <template #header>
            <div class="card-header">
                <span>网络负载</span>
            </div>
        </template>
        <v-chart class="chart" ref="networkUsageCharts" :manual-update="true" autoresize />
    </el-card>
</template>

<style scoped>
.chart {
    height: 100px;
    width: 100%;
}

.el-row:last-child {
    margin-bottom: 0;
}

.el-card {
    --el-card-padding: 8px
}

.card-header {
    font-size: smaller;
    /* font-weight: bold; */
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--el-text-color-regular);
}
</style>