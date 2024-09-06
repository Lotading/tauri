// place files you want to import through the `$lib` alias in this folder.
import { fetch } from "@tauri-apps/api/http";

const response = await fetch("http://test.tauri.app/data.json", {
  method: 'GET',
});

console.log(response.status)
