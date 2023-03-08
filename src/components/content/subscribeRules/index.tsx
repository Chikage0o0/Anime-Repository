import { useStore } from "@/store";
import {
  ActionIcon,
  Affix,
  Anchor,
  Button,
  Center,
  Group,
  Popover,
  ScrollArea,
  Table,
  Text,
  TextInput,
  useMantineTheme,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconCheck,
  IconPencil,
  IconPlus,
  IconSearch,
  IconTrash,
  IconX,
} from "@tabler/icons-react";
import { flowResult } from "mobx";
import { observer } from "mobx-react-lite";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import EditRule from "./editRule";

function getLink(provider: string, id: string): string {
  switch (provider) {
    case "imdb": {
      return `https://www.imdb.com/title/${id}`;
    }
    case "tmdb": {
      return `https://www.themoviedb.org/tv/${id}`;
    }
    default: {
      return "";
    }
  }
}

function SubscribeRules() {
  const { t } = useTranslation();
  const { settingStore, subscribeRulesStore: subscribeStore } = useStore();
  const [opened, setOpened] = useState(false);
  const [search, setSearch] = useState("");
  const form = useForm({
    initialValues: {
      title: "",
      id: "",
      provider: settingStore.setting.scraper.default_provider,
      tvshow_regex: "",
      season: 1,
      lang: settingStore.setting.scraper.default_lang,
      episode_regex: "\\d+",
      episode_position: 1,
      episode_offset: 0,
    },
    validate: {
      id: (value) => {
        if (!value) {
          return t("subscribe_rules.id_required");
        }
      },
      tvshow_regex: (value) => {
        if (!value) {
          return t("subscribe_rules.tvshow_regex_required");
        }
        try {
          new RegExp(value);
        } catch (e) {
          return t("subscribe_rules.tvshow_regex_invalid");
        }
      },
      season: (value) => {
        if (value < 0) {
          return t("subscribe_rules.season_invalid");
        }
      },
      lang: (value) => {
        if (!/^[a-z]{2}-[A-Z]{2}$/g.test(value)) {
          return t("subscribe_rules.lang_invalid");
        }
      },
      episode_regex: (value) => {
        if (!value) {
          return t("subscribe_rules.episode_regex_required");
        }
        try {
          new RegExp(value);
        } catch (e) {
          return t("subscribe_rules.episode_regex_invalid");
        }
      },
      episode_position: (value) => {
        if (value < 1) {
          return t("subscribe_rules.episode_position_invalid");
        }
      },
    },
  });

  const theme = useMantineTheme();

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = event.currentTarget;
    setSearch(value);
  };

  const data = subscribeStore.data
    .filter((item) => {
      if (search === "") {
        return true;
      }
      return item.id.includes(search) || item.title.includes(search);
    })
    .map((item) => (
      <tr key={item.provider + item.id}>
        <td>
          <Group>
            <Text size="sm">{item.provider.toUpperCase() + ":"}</Text>
            <Anchor
              onClick={() =>
                window.open(getLink(item.provider, item.id), "_blank")
              }
            >
              {item.id}
            </Anchor>
          </Group>
        </td>
        <td width="60%">
          <Text
            size="sm"
            weight={500}
            style={{ wordBreak: "break-all", wordWrap: "break-word" }}
          >
            {item.title}
          </Text>
        </td>
        <td>
          <Text size="sm">{item.season}</Text>
        </td>
        <td>
          <Text size="sm">{item.lang}</Text>
        </td>
        <td style={{ maxWidth: 50 }}>
          <Group spacing={0} position="right">
            <ActionIcon
              onClick={() => {
                form.setValues(item);
                setOpened(true);
              }}
            >
              <IconPencil size={16} stroke={1.5} />
            </ActionIcon>
            <Popover width={200} position="bottom" withArrow shadow="md">
              <Popover.Target>
                <ActionIcon color="red">
                  <IconTrash size={16} stroke={1.5} />
                </ActionIcon>
              </Popover.Target>
              <Popover.Dropdown>
                <Text size="sm">{t("UI.delete_confirm")}</Text>
                <Center>
                  <Button
                    variant="outline"
                    color="red"
                    radius="xs"
                    size="xs"
                    mt="xs"
                    onClick={() =>
                      flowResult(
                        subscribeStore.delSubscribeRule(item.id, item.provider)
                      )
                        .then(() => {
                          notifications.show({
                            icon: <IconCheck />,
                            title: t("subscribe_rules.delete_success"),
                            message: "✌️🙄✌️",
                          });
                        })
                        .catch((e) => {
                          notifications.show({
                            color: "red",
                            icon: <IconX />,
                            autoClose: false,
                            title: t("subscribe_rules.delete_failed"),
                            message: e,
                          });
                        })
                    }
                    compact
                  >
                    {t("UI.true")}
                  </Button>
                </Center>
              </Popover.Dropdown>
            </Popover>
          </Group>
        </td>
      </tr>
    ));

  return (
    <div style={{ padding: 30 }}>
      <TextInput
        placeholder={t("subscribe_rules.search_by_id_or_title") as string}
        mb="xs"
        onChange={handleSearchChange}
        icon={<IconSearch size={14} stroke={1.5} />}
      />
      <ScrollArea style={{ height: "calc(100vh - 110px)" }} type="scroll">
        <Table verticalSpacing="sm" striped highlightOnHover>
          <thead>
            <tr>
              <th>{t("subscribe_rules.ID")}</th>
              <th>{t("subscribe_rules.title")}</th>
              <th>{t("subscribe_rules.season")}</th>
              <th>{t("subscribe_rules.lang")}</th>
              <th />
            </tr>
          </thead>
          <tbody>{data}</tbody>
        </Table>
      </ScrollArea>
      <Affix
        hidden={settingStore.menu_open}
        position={{ bottom: 20, right: 20 }}
      >
        <ActionIcon
          size="xl"
          radius="xl"
          variant="filled"
          color={theme.primaryColor}
          onClick={() => setOpened(true)}
        >
          <IconPlus stroke={1.5} size={34} />
        </ActionIcon>
      </Affix>
      <EditRule opened={opened} setOpened={setOpened} form={form} />
    </div>
  );
}

export default observer(SubscribeRules);
