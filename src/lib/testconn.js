import { fetch } from "@tauri-apps/api/http";
import { invoke } from "@tauri-apps/api";

export const testconn = async () => {
  try {
    const port = await invoke('get_port')
    const response = await fetch(`http://localhost:${port}/`)
    console.log(response)
  } catch (error){
    console.error(error)
  }
}

testconn();
