import { Divider, NumberInput, TextInput } from "@mantine/core";
import { IconFolder } from "@tabler/icons-react";
import { open } from "@tauri-apps/api/dialog";
import { useTranslation } from "react-i18next";

function Storage({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation();
  const selected = async (name: string, default_path: string) => {
    const res = await open({
      directory: true,
      multiple: false,
      defaultPath: default_path,
    });
    if (res != null) form.setFieldValue(name, res);
  };

  return (
    <>
      <Divider
        my="md"
        label={t("setting.storage")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <TextInput
        autoComplete="off"
        className={classes.input}
        label={t("setting.storage.pending_path")}
        {...form.getInputProps("storage.pending_path")}
        rightSection={
          <IconFolder
            stroke={1}
            onClick={() =>
              selected(
                "storage.pending_path",
                form.values["storage"]["pending_path"]
              )
            }
          />
        }
      />
      <TextInput
        autoComplete="off"
        className={classes.input}
        label={t("setting.storage.repository_path")}
        {...form.getInputProps("storage.repository_path")}
        rightSection={
          <IconFolder
            stroke={1}
            onClick={() =>
              selected(
                "storage.repository_path",
                form.values["storage"]["repository_path"]
              )
            }
          />
        }
      />
    </>
  );
}

export default Storage;
