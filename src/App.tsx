/*
 * @Author: your name
 * @Date: 2021-12-23 20:14:22
 * @LastEditTime: 2022-01-13 16:06:52
 * @LastEditors: Please set LastEditors
 * @Description: 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 * @FilePath: /scaffold-demo/tauri-app/tauri-app/src/App.tsx
 */

import React, { FC, useState } from 'react'
import './App.css'

import { Button, message } from 'antd';
import { SearchOutlined, FolderOpenOutlined, SyncOutlined } from '@ant-design/icons';

// With the Tauri API npm package:
import { dialog, shell } from '@tauri-apps/api'
import { invoke } from '@tauri-apps/api/tauri'

const JSON_FILE_NAME = "setting.json";
const APP_NAME = `./Library/Containers/com.tencent.WeWorkMac/Data/Documents/Profiles/${JSON_FILE_NAME}`;

const App: FC = () => {
  const [loading, setLoading] = useState(false);
  const [exportLoading, setExportLoading] = useState(false);
  const [appFolderPath, setAppFolderPath] = useState<string>();
  const [exportFolderPath, setExportFolderPath] = useState<string>();
  const startSearch = async () => {
    setLoading(true);
    // 调用tauri注册的方法
    const jsonStr = await invoke('my_custom_command', { invokeMessage: APP_NAME });
    const { path, code} = JSON.parse(`${jsonStr}`);
    const folderPath = `${path.replace(JSON_FILE_NAME, '')}${code}/Caches/Emotions`
    if (code) {
      setAppFolderPath(folderPath);
    } else {
      message.error('未找到应用地址');
    }
    setLoading(false);
  }

  // 选择要导出的文件夹 & 执行导出
  const exportEmojiByPathSelected = async () => {
    const tmpPath = await dialog.open({ directory: true, multiple: false })
    const exportPath = `${tmpPath}/wvx-export-folder_${Date.now()}`
    if (exportPath) {
      setExportLoading(true);
      const res = await invoke('search_folder_by_path', { folderPath: appFolderPath, exportPath });
      if (res) {
        message.success('导出成功');
        setExportFolderPath(Array.isArray(exportPath) ? exportPath[0] : exportPath)
      }
      setExportLoading(false)
    }
  }

  return (
    <div className="App">
      <div className="start">
        <div>
          {appFolderPath ? <div>
            <Button icon={exportLoading? <SyncOutlined spin /> :<FolderOpenOutlined />} loading={exportLoading} type="primary" size='large' onClick={() => exportEmojiByPathSelected()}>选择导出到文件夹</Button>
            {exportFolderPath ? <p className="path-select-open" onClick={()=>shell.open(exportFolderPath)}>{ `📂点击打开👉${exportFolderPath}` }</p>:null}
          </div>: <Button icon={<SearchOutlined />} loading={loading} type="primary" size='large' onClick={() => startSearch()}>开始搜索</Button>}
        </div>
      </div>
    </div>
  )
}

export default App
