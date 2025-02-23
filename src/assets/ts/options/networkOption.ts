import { EChartsType } from "echarts/core";
import { NetworkData } from "../monitor";

const normalSpliteLine = {
    show: true,
    lineStyle: {
        color: '#4C4D4F',
        opacity: .5,
    }
};

export const networkOption = {
    grid: {
        left: '4',
        right: '4',
        bottom: '4',
        top: '4',
        containLabel: true
    },
    xAxis: [{
        type: 'category',
        data: [],
        splitLine: normalSpliteLine,
        axisLabel: {
            show: true,
        },
        axisPointer: {
            show: false,
        },
        axisTick: {
            show: false,
        }
    }],
    yAxis: {
        type: 'value',
        min: 0,
        splitLine: normalSpliteLine,
        axisLabel: {
            show: false,
        },
        axisTick: {
            show: false,
        },
        axisPointer: {
            value: 0,
            snap: true,
            show: false,
            label: {
                formatter: function (params: any): string {
                    return params.value.toFixed(0) + 'KB/s';
                }
            },
            handle: {
                show: true,
            }
        }
    },
    series: [
        {
            name: 'Upload Speed',
            type: 'line',
            smooth: true,
            showSymbol: false,
            lineStyle: {
                color: '#FF5733', // 上传速率的线颜色
            },
            areaStyle: {
                color: 'rgba(255, 87, 51, 0.3)', // 上传速率的区域填充颜色
            },
            markLine: {
                symbol: 'none',
                label: {
                    formatter: function (params: any): string {
                        return params.value.toFixed(0) + 'KB/s';
                    },
                    position: 'insideEndBottom',
                    color: '#E5EAF3'
                },
                data: [
                    {
                        type: 'max',
                    }
                ]
            },
            data: []
        },
        {
            name: 'Download Speed',
            type: 'line',
            smooth: true,
            showSymbol: false,
            lineStyle: {
                color: '#33A1FF', // 下载速率的线颜色
            },
            areaStyle: {
                color: 'rgba(51, 161, 255, 0.3)', // 下载速率的区域填充颜色
            },
            markLine: {
                symbol: 'none',
                label: {
                    formatter: function (params: any): string {
                        return params.value.toFixed(0) + 'KB/s';
                    },
                    position: 'insideEndBottom',
                    color: '#E5EAF3'
                },
                data: [
                    {
                        type: 'max',
                    }
                ]
            },
            data: []
        },
    ]
};

export function setNetworkGuage(echart: EChartsType, networkData: NetworkData) {
    const MAX_DATA_POINTS = 30;

    const currentOption = echart.getOption();
    let uploadSeries = [];
    let downloadSeries = [];
    if (Array.isArray(currentOption.series) && currentOption.series.length > 1) {
        uploadSeries = currentOption.series[0].data || []
        downloadSeries = currentOption.series[1].data || []
    }
    uploadSeries.push([networkData.timestamp, networkData.transmitted_rate / 1024]);
    downloadSeries.push([networkData.timestamp, networkData.received_rate / 1024]);

    if (uploadSeries.length > MAX_DATA_POINTS) {
        uploadSeries = uploadSeries.slice(-MAX_DATA_POINTS);
    }
    if (downloadSeries.length > MAX_DATA_POINTS) {
        downloadSeries = downloadSeries.slice(-MAX_DATA_POINTS);
    }
    echart.setOption({
        series: [
            { data: uploadSeries },
            { data: downloadSeries }
        ]
    });
}
