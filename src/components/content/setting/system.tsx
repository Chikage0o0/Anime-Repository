import { useTranslation } from "react-i18next";
import { Divider, Group, NumberInput, Switch } from "@mantine/core";
import { useState } from "react";

function System({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation();
  const [autoLaunch, setAutoLaunch] = useState(form.values.system.auto_launch);
  const [silentStart, setSilentStart] = useState(
    form.values.system.silent_start
  );

  return (
    <>
      <Divider
        my="md"
        label={t("setting.system")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <Group position="center" grow className={classes.input}>
        <Switch
          checked={silentStart}
          onChange={(event) => {
            setSilentStart(event.currentTarget.checked);
            form.setFieldValue(
              "system.silent_start",
              event.currentTarget.checked
            );
          }}
          size="sm"
          label={t("setting.system.silent_start")}
        />
        <Switch
          checked={autoLaunch}
          onChange={(event) => {
            setAutoLaunch(event.currentTarget.checked);
            form.setFieldValue(
              "system.auto_launch",
              event.currentTarget.checked
            );
          }}
          size="sm"
          label={t("setting.system.auto_launch")}
        />
      </Group>

      <NumberInput
        className={classes.input}
        label={t("setting.system.scan_interval")}
        {...form.getInputProps("system.scan_interval")}
      />
    </>
  );
}

export default System;
