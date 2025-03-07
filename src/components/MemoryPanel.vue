<script setup lang="ts">
import { ref, onMounted, provide } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import 'echarts';
import 'echarts/theme/blue'
import VChart, { THEME_KEY } from 'vue-echarts';
import { MemoryData } from "../assets/ts/monitor";
import { memoryOption } from "../assets/ts/options/options";
import { EChartsType } from "echarts/core";
provide(THEME_KEY, 'blue')

const totalMemory = ref<string>("16");
const memoryChart = ref<EChartsType>();

function updateMemoryUsage(memory: MemoryData) {
    memoryChart.value?.setOption({
        xAxis: {
            max: Number.parseInt(totalMemory.value),
            interval: Math.floor(Number.parseInt(totalMemory.value) / 16) * 4,
        },
        dataset: {
            source: [['memory', (memory.used_memory + memory.used_swap).toFixed(1), memory.total_memory, memory.total_swap]]
        }
    });
}

function flushMemoryUsage() {
    invoke<MemoryData>("memory_info", {}).then(memory => {
        totalMemory.value = (memory.total_memory + memory.total_swap).toFixed(0);
        updateMemoryUsage(memory);
    });
    return flushMemoryUsage;
}

onMounted(async () => {
    memoryChart.value?.setOption(memoryOption);
    setInterval(flushMemoryUsage(), 10 * 1000);
})

</script>

<template>
    <el-row>
        <el-col>
            <el-card>
                <template #header>
                    <div class="card-header">
                        <span style="width: 40px">内存</span>
                    </div>
                </template>
                <v-chart class="chart" ref="memoryChart" :manual-update="true" autoresize />
            </el-card>
        </el-col>
    </el-row>
</template>

<style scoped>
.chart {
    height: 33px;
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

.card-header {
    font-size: smaller;
    /* font-weight: bold; */
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--el-text-color-regular);
}
</style>