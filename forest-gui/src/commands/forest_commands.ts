import { invoke } from "@tauri-apps/api";
import { InvokeArgs } from "@tauri-apps/api/tauri";
import { IForestInfo } from "../types/response";

export interface ICreateForestArgs extends InvokeArgs {
  seed?: number;
  width?: number;
  height?: number;
}

export async function create_forest(args: ICreateForestArgs): Promise<IForestInfo> {
  try {
    return await invoke<IForestInfo>('create_forest', args);
  } catch (err) {
    throw err;
  }
}

export async function get_forest(): Promise<IForestInfo> {
  try {
    return await invoke<IForestInfo>('get_forest');
  } catch (err) {
    throw err;
  }
}

export async function update_forest(): Promise<void> {
  try {
    return await invoke<void>('update_forest');
  } catch (err) {
    throw err;
  }
}
