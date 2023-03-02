import { useTranslation } from "react-i18next";
import { Divider, Switch, TextInput } from "@mantine/core";
import { useState } from "react";

function Network({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation();
  const [checked, setChecked] = useState(form.values.network.use_proxy);

  return (
    <>
      <Divider
        my="md"
        label={t("setting.network")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />

      <Switch
        checked={checked}
        className={classes.input}
        onChange={(event) => {
          setChecked(event.currentTarget.checked);
          form.setFieldValue("network.use_proxy", event.currentTarget.checked);
        }}
        size="sm"
        label={t("setting.network.use_proxy")}
      />

      <TextInput
        className={classes.input}
        label={t("setting.network.proxy")}
        placeholder="socks5://127.0.0.1:1080"
        {...form.getInputProps("network.proxy")}
      />
    </>
  );
}

export default Network;
