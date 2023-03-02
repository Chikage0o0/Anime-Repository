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
import { Developer, developer_list } from "./about";
import { getVersion } from "@tauri-apps/api/app";
const appVersion = await getVersion();

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
          if (
            !(
              value === "" ||
              /^(?:http(?:s?)|socks(?:5|5h)):\/\/(?:[A-Za-z0-9]*:[A-Za-z0-9]*@)*(?:\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(?:\d{2,5})$/.test(
                value
              )
            )
          ) {
            return t("setting.network.proxy_error");
          }
        },
      },
    },
  });

  const developer = developer_list.map((item) => {
    return <Developer {...item} />;
  });

  return (
    <ScrollArea style={{ height: "100vh", padding: 30 }} type="scroll">
      <UI form={form} classes={classes} />
      <Storage form={form} classes={classes} />
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
      <Divider
        my="md"
        label={t("setting.about")}
        labelProps={{
          component: "p",
          style: { fontSize: 16, fontWeight: 500 },
        }}
        labelPosition="center"
      />

      <Flex
        className={classes.input}
        gap="md"
        justify="flex-start"
        align="flex-start"
        direction="row"
        wrap="wrap"
      >
        {developer}
      </Flex>

      <Stack align="center" spacing={0}>
        <Group position="center">
          <Text
            size="sm"
            sx={(theme) => ({
              color: theme.colors.gray[6],
            })}
          >
            Copyright © Anime Repository Develop Team 2023
          </Text>
          <IconBrandGithub
            onClick={() => {
              window.open(
                "https://github.com/Chikage0o0/Anime-Repository",
                "_blank"
              );
            }}
            size={16}
          />
        </Group>
        <Text
          size="sm"
          sx={(theme) => ({
            color: theme.colors.gray[6],
          })}
        >
          Anime Repository is licensed under the GNU General Public License v3.0
        </Text>
        <Text
          size="sm"
          sx={(theme) => ({
            color: theme.colors.gray[6],
          })}
        >
          Version: {appVersion}
        </Text>
      </Stack>
    </ScrollArea>
  );
}

export default observer(Setting);
