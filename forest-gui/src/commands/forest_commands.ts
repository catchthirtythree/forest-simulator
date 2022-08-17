import { invoke } from "@tauri-apps/api";

export async function get_forest(): Promise<number[]> {
  try {
    return await invoke<number[]>('get_forest');
  } catch (err) {
    console.error(err);
    return [];
  }
}

export async function update_forest(): Promise<void> {
  try {
    return await invoke<void>('update_forest');
  } catch (err) {
    console.error(err);
    return Promise.reject();
  }
}
