import { fetch } from "@tauri-apps/api/http";
import { invoke } from "@tauri-apps/api";

export const testconn = async () => {
  try {
    const port = await invoke('get_port')
    const response = await fetch(`http://localhost:${port}/`)
    return response
  } catch (error){
    console.error(error)
    return error;
  }
}

export const get_port = () => {
  let port = invoke('get_port');
  return port;
}