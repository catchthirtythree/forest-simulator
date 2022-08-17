import { invoke } from "@tauri-apps/api";

export async function get_map(): Promise<number[]> {
  try {
    return await invoke<number[]>('get_map');
  } catch (err) {
    console.error(err);
    return [];
  }
}

export async function update_map(): Promise<number[]> {
  try {
    return await invoke<number[]>('get_map');
  } catch (err) {
    console.error(err);
    return [];
  }
}
