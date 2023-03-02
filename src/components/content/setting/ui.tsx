import setting from "@/store/settingStore";
import i18n from "i18next";
import { useTranslation } from "react-i18next";
import {
  ColorSwatch,
  Divider,
  Group,
  Select,
  useMantineTheme,
} from "@mantine/core";
import { locales } from "@/locales/locales";
import { useStore } from "@/store";

function UI({ form, classes }: { form: any; classes: any }) {
  const { settingStore } = useStore();
  const { t } = useTranslation();
  const lang = locales.map((item) => ({
    value: item,
    label: t(`lang.${item}`),
  }));
  const theme = useMantineTheme();
  const swatches = Object.keys(theme.colors).map((color) => (
    <ColorSwatch
      key={color}
      color={theme.colors[color][6]}
      onClick={() => {
        form.setFieldValue("ui.primary_color", color);
        settingStore.changePrimaryColor(color);
      }}
    />
  ));

  const UITheme = [
    {
      value: "Auto",
      label: t("setting.ui.theme.auto"),
    },
    {
      value: "Light",
      label: t("setting.ui.theme.light"),
    },
    {
      value: "Dark",
      label: t("setting.ui.theme.dark"),
    },
  ];

  return (
    <>
      <Divider
        my="md"
        label={t("setting.ui")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />
      <Select
        data={lang}
        className={classes.input}
        label={t("setting.ui.lang") + ":"}
        radius="sm"
        {...form.getInputProps("ui.lang")}
        onChange={(event) => {
          form.setFieldValue("ui.lang", event);
          i18n.changeLanguage(event as string);
        }}
      />
      <Select
        className={classes.input}
        data={UITheme}
        label={t("setting.ui.theme") + ":"}
        radius="sm"
        {...form.getInputProps("ui.theme")}
        onChange={(event) => {
          form.setFieldValue("ui.theme", event);
          setting.changeTheme(event as string);
        }}
      />
      <Group position="center" spacing="xs" className={classes.input}>
        {swatches}
      </Group>
    </>
  );
}

export default UI;
