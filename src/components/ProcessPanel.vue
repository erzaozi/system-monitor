<script setup lang="ts">
import { ref, onMounted, provide } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import 'echarts';
import 'echarts/theme/blue'
import VChart, { THEME_KEY } from 'vue-echarts';
import { ProcessData } from "../assets/ts/monitor";
import { processOption } from "../assets/ts/options/options";
import { EChartsType } from "echarts/core";
provide(THEME_KEY, 'blue')

const processChart = ref<EChartsType>();
let processesList = ref<ProcessData[]>([]);
const showNum = ref<number>(10);

function updateProcessMem(processes: ProcessData[]) {
    const source = []
    for (const key in processes) {
        source.push([processes[key].pid, processes[key].name, (processes[key].memory * 1024.).toFixed(0)])
    }
    processChart.value?.setOption({
        dataset: {
            source: source
        }
    });
}

function increment() {
    console.log(111)
    showNum.value++;
    updateProcessMem(processesList.value.slice(0, showNum.value).reverse());
}

function decrement() {
    if (showNum.value > 0) {
        showNum.value--;
    }
    updateProcessMem(processesList.value.slice(0, showNum.value).reverse());
}

function flushProcessData() {
    invoke<ProcessData[]>("process_info", {}).then(processes => {
        processesList.value = processes;
        updateProcessMem(processesList.value.slice(0, showNum.value).reverse());
    })
    return flushProcessData
}

onMounted(async () => {
    processChart.value?.setOption(processOption);
    setInterval(flushProcessData(), 1000);
})




</script>

<template>
    <el-button-group style="width: 100%; ">
        <el-button type="primary" @click ="increment()">+</el-button>
        <el-button type="primary" @click ="decrement()">-</el-button>
    </el-button-group>
    <el-row>
        <el-col :span="24">
            <el-card>
                <v-chart class="chart" ref="processChart" :manual-update="true" autoresize />
            </el-card>
        </el-col>
    </el-row>

</template>

<style scoped>
.chart {
    height: 290px;
    width: 100%;
}

.el-row {
    margin-bottom: 8px;
}

.el-row:last-child {
    margin-bottom: 0;
}

.el-card {
    --el-card-padding: 8px
}

.el-button {
    width: 50%;
    z-index: 10000;
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