import { useTranslation } from "react-i18next";
import { Divider, NumberInput } from "@mantine/core";

function System({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation();

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

      <NumberInput
        className={classes.input}
        label={t("setting.system.scan_interval")}
        {...form.getInputProps("system.scan_interval")}
      />
    </>
  );
}

export default System;
