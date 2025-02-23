<script setup lang="ts">
import { ref, reactive, onMounted, Ref, provide, onUnmounted } from "vue";
import 'echarts';
import 'echarts/theme/blue'
import VChart, { THEME_KEY } from 'vue-echarts';
import { gaugeOption } from "../assets/ts/options/options";
import { setSpecialGuage } from "../assets/ts/options/gaugeOption";
import BatteryPanel from "./BatteryPanel.vue";
import CpuPanel from "./CpuPanel.vue";
import ProcessPanel from "./ProcessPanel.vue";
import MemoryPanel from "./MemoryPanel.vue";
import GpuPanel from "./GpuPanel.vue";
import NetworkPanel from "./NetworkPanel.vue";
import { EChartsType } from "echarts/core";
provide(THEME_KEY, 'blue')

const payloadChart = ref<EChartsType>();
const cpuUpdateLoad = (msg: Number) => {
    setSpecialGuage(payloadChart.value!, "负载", msg.toFixed(2));
}

onMounted(async () => {
    payloadChart.value?.setOption(gaugeOption);
})

onUnmounted(async () => { })

</script>

<template>
    <CpuPanel @message="cpuUpdateLoad" />
    <el-row :gutter="8">
        <el-col :span="12">
            <BatteryPanel />
        </el-col>
        <el-col :span="12">
            <MemoryPanel />
        </el-col>
    </el-row>
    <el-row :gutter="8">
        <el-col :span="6">
            <el-card>
                <template #header>
                    <div class="card-header">
                        <span>系统负载</span>
                    </div>
                </template>
                <v-chart class="chart" ref="payloadChart" :manual-update="true" autoresize />
            </el-card>
        </el-col>
        <el-col :span="6">
            <GpuPanel />
        </el-col>
        <el-col :span="12">
            <NetworkPanel />
        </el-col>
    </el-row>
    <ProcessPanel />

</template>

<style scoped>

.chart {
    height: 100px;
    width: 100%;
}

.el-row {
    margin-bottom: 8px;
}

.el-row:last-child {
    margin-bottom: 0;
}

:deep() .el-card {
    --el-card-padding: 8px;
    --el-card-border-radius: 10px;
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