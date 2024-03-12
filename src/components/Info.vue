<template>
    <div class="full-fill">
        <div class="common-layout full fc">
            <el-container class="full">
                <el-aside class="full-height" width="200px">
                    <el-col class="full-height fc">
                        <el-input v-model="input" placeholder="Please input">
                            <template #append>
                                <el-button :icon="CirclePlus" @click="newExt" />
                            </template>
                        </el-input>
                        <el-auto-resizer>
                            <template #default="{ height, width }">
                                <el-table-v2 :columns="columns" :data="headers" :width="width" :header-height=0
                                    :row-class="rowClass" :row-event-handlers="rowClick" :height="height" />
                            </template>
                        </el-auto-resizer>
                    </el-col>
                </el-aside>
                <el-main>
                    <el-form :model="ext" label-width="auto" style="max-width: 600px">
                        <el-form-item label="Name">
                            <el-input v-model="ext.name" />
                        </el-form-item>
                        <el-form-item label="Prompt">
                            <el-input v-model="ext.prompt" />
                        </el-form-item>
                        <el-form-item label="Directory">
                            <el-input v-model="ext.dir" />
                        </el-form-item>
                        <el-form-item label="Exec">
                            <el-input v-model="ext.exec" />
                        </el-form-item>
                        <el-form-item label="Addr">
                            <el-input v-model="ext.addr" />
                        </el-form-item>
                        <el-form-item>
                            <el-button type="primary" @click="extSave">Save</el-button>
                            <el-button @click="extDel">Delete</el-button>
                        </el-form-item>
                    </el-form>
                </el-main>
            </el-container>
        </div>
    </div>
</template>


<script lang="ts" setup>
import { ref, onMounted, watch } from 'vue'
import { CirclePlus } from '@element-plus/icons-vue'
import { ElButton, } from 'element-plus'
import { invoke } from '@tauri-apps/api'
import type { RowEventHandlerParams, Column, RowClassNameGetter } from 'element-plus'


const errorAssert = (err: unknown) => {
    if (typeof err === 'string') {
        console.log(err)
        return false
    }
    return true
}


const input = ref('')
const columns: Column<any>[] = [
    {
        key: 'id',
        dataKey: 'name',
        width: 150,
    },
]
type ExtHeader = {
    id: string
    name: string
}
var headers = ref<ExtHeader[]>([])
onMounted(async () => {
    let res = await invoke('get_headers');
    if (errorAssert(res)) {
        headers.value = res as ExtHeader[]
    }
})

type Ext = {
    id: string
    name: string
    prompt: string
    dir: string
    exec: string
    addr: string
}
var unsaved = ref<Map<number, Ext>>(new Map())

var ext = ref({
    id: '',
    name: '',
    prompt: '',
    dir: '',
    exec: '',
    addr: '',
})

var cur_id = ref('')
var cur_index = ref(-1)
var switch_flag = ref(false)


const rowClick = {
    onClick({ rowIndex, rowKey }: RowEventHandlerParams) {
        cur_id.value = rowKey as string
        cur_index.value = rowIndex
        let his_ext = unsaved.value.get(rowIndex)
        if (his_ext !== undefined) {
            switch_flag.value = true
            ext.value = his_ext
            return
        }
        invoke('get_ext', { id: rowKey }).then((res) => {
            if (errorAssert(res)) {
                switch_flag.value = true
                ext.value = res as Ext;
            }
        })
    },
}

watch(() => ext.value, (newVal) => {
    if (!switch_flag.value) {
        headers.value[cur_index.value].name = newVal.name
        unsaved.value.set(cur_index.value, newVal)
    }
    switch_flag.value = false
}, { deep: true })

const newExt = () => {
    headers.value.push({ id: 'new', name: '' })
    cur_id.value = 'new'
    cur_index.value = headers.value.length - 1
    ext.value = {
        id: '',
        name: '',
        prompt: '',
        dir: '',
        exec: '',
        addr: '',
    }
}
type ExtId = {
    id: string
}
const extSave = async () => {
    if (cur_index.value === -1) {
        return
    }
    let res = await invoke('set_ext', { id: cur_id.value, ext: ext.value });
    if (errorAssert(res)) {
        unsaved.value.delete(cur_index.value)
        cur_id.value = (res as ExtId).id
        headers.value[cur_index.value].id = cur_id.value;
    }
}
const extDel = async () => {
    if (cur_index.value === -1) {
        return
    }
    let res = await invoke('del_ext', { id: cur_id.value });
    if (errorAssert(res)) {
        headers.value.splice(cur_index.value, 1)
        unsaved.value.delete(cur_index.value)
        cur_id.value = ''
        cur_index.value = -1
    }
}
const rowClass = ({ rowIndex }: Parameters<RowClassNameGetter<any>>[0]) => {
    if (unsaved.value.has(rowIndex)) {
        return 'dot-after'
    }
    return ''
}

</script>
<style>
.full {
    height: 100%;
    width: 100%;
}

.full-height {
    height: 100%;
}

.fc {
    display: flex;
    flex-direction: column;
}

.full-fill {
    height: 97vh;
    width: 97vw;
}

.dot-after::after {
    /* content: "\00B7"; */
    content: "(unsaved)";
    font-size: 1em;
    /* Unicode 点号 */
    /* margin-left: 0.25em; */
    /* 可以调整点号与文本之间的间距 */
}
</style>