import { useTranslation } from "react-i18next";
import {
  Divider,
  Group,
  PasswordInput,
  Select,
  Switch,
  TextInput,
} from "@mantine/core";
import { useState } from "react";

function Scraper({ form, classes }: { form: any; classes: any }) {
  const { t } = useTranslation();
  const [checked, setChecked] = useState(form.values.scraper.use_openai);

  return (
    <>
      <Divider
        my="md"
        label={t("setting.scraper")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <Group position="center" mb="xs" grow className={classes.input}>
        <TextInput
          autoComplete="off"
          label={t("setting.scraper.default_lang")}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps("scraper.default_lang")}
        />
        <Select
          label={t("setting.scraper.default_provider")}
          data={[{ value: "tmdb", label: "TMDB" }]}
          {...form.getInputProps("scraper.default_provider")}
        />
      </Group>

      <Switch
        checked={checked}
        className={classes.input}
        onChange={(event) => {
          setChecked(event.currentTarget.checked);
          form.setFieldValue("scraper.use_openai", event.currentTarget.checked);
        }}
        size="sm"
        label={t("setting.scraper.use_openai")}
      />

      <PasswordInput
        className={classes.input}
        label={t("setting.scraper.openai_key")}
        placeholder="sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        {...form.getInputProps("scraper.openai_key")}
      />
    </>
  );
}

export default Scraper;
