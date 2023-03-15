import { invoke } from "@tauri-apps/api";

export const getTitle = async (
  id: string,
  provider: string,
  lang: string,
  type: string
) => {
  if (id && provider && lang && type) {
    try {
      const res = await invoke("get_title", {
        id: id,
        provider: provider,
        lang: lang,
        type: type,
      });
      return res;
    } catch (error: any) {
      throw new Error(error);
    }
  }
};
