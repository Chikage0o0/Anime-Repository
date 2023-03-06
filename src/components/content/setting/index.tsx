import { useStore } from "@/store";
import { SettingObject } from "@/store/settingStore";
import {
  ActionIcon,
  Affix,
  createStyles,
  Divider,
  Text,
  Flex,
  ScrollArea,
  useMantineTheme,
  Group,
  Stack,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconBrandGithub,
  IconCheck,
  IconDeviceFloppy,
  IconX,
} from "@tabler/icons-react";
import { flowResult } from "mobx";
import { observer } from "mobx-react-lite";
import { useTranslation } from "react-i18next";
import Network from "./network";
import Storage from "./storage";
import UI from "./ui";
import { useForceUpdate } from "@mantine/hooks";
import System from "./system";

import { getVersion } from "@tauri-apps/api/app";
import Scraper from "./scraper";
import About from "./about";

const useStyles = createStyles(() => {
  return {
    input: {
      paddingBottom: 18,
      paddingLeft: 20,
      paddingRight: 20,
    },
  };
});

function Setting() {
  const { t } = useTranslation();
  const theme = useMantineTheme();
  const { classes } = useStyles();
  const { settingStore } = useStore();
  const forceUpdate = useForceUpdate();
  const form = useForm({
    initialValues: settingStore.setting,
    validate: {
      network: {
        proxy: (value: string) => {
          if (form.values.network.use_proxy) {
            if (
              !/^(?:http(?:s?)|socks(?:5|5h)):\/\/(?:[A-Za-z0-9]*:[A-Za-z0-9]*@)*(?:\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(?:\d{2,5})$/.test(
                value
              )
            ) {
              return t("setting.network.proxy_error");
            }
          }
        },
      },
      system: {
        scan_interval: (value: number) => {
          if (value < 1) {
            return t("setting.system.scan_interval_error");
          }
        },
      },
      scraper: {
        default_lang: (value: string) => {
          if (!/^[a-z]{2}-[A-Z]{2}$/.test(value)) {
            return t("setting.scraper.default_lang_error");
          }
        },
        openai_key: (value: string) => {
          if (form.values.scraper.use_openai) {
            if (!value.startsWith("sk-")) {
              return t("setting.scraper.openai_key_error");
            }
          }
        },
      },
    },
  });

  return (
    <ScrollArea style={{ height: "100vh", padding: 30 }} type="scroll">
      <UI form={form} classes={classes} />
      <Storage form={form} classes={classes} />
      <Scraper form={form} classes={classes} />
      <Network form={form} classes={classes} />
      <System form={form} classes={classes} />
      <Affix
        hidden={settingStore.menu_open}
        position={{ bottom: 20, right: 20 }}
      >
        <ActionIcon
          size="xl"
          radius="xl"
          variant="filled"
          loading={settingStore.loading}
          color={theme.primaryColor}
          onClick={() => {
            if (!form.validate().hasErrors) {
              flowResult(
                settingStore.applySetting(form.values as SettingObject)
              )
                .then(() => {
                  notifications.show({
                    icon: <IconCheck />,
                    title: t("setting.save_success"),
                    message: "✌️🙄✌️",
                  });
                })
                .catch((e) => {
                  forceUpdate();
                  notifications.show({
                    color: "red",
                    icon: <IconX />,
                    autoClose: false,
                    title: t("setting.save_failed"),
                    message: e,
                  });
                });
            }
          }}
        >
          <IconDeviceFloppy stroke={1.2} size={34} />
        </ActionIcon>
      </Affix>
      <About classes={classes} />
    </ScrollArea>
  );
}

export default observer(Setting);
