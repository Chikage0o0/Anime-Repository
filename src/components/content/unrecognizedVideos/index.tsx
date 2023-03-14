import { useStore } from "@/store";
import { unrecognizedVideoObject } from "@/store/unrecognizedStore";
import {
  ActionIcon,
  Affix,
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
  IconHeartPlus,
  IconPencil,
  IconRefresh,
  IconSearch,
  IconTrash,
  IconX,
} from "@tabler/icons-react";
import { flowResult } from "mobx";
import { observer } from "mobx-react-lite";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import EditVideo from "./editVideo";
import InsertRule from "./insertRule";

function UnrecognizedVideos() {
  const { t } = useTranslation();
  const { settingStore, unrecognizedVideosStore } = useStore();
  const [editVideoOpened, setEditVideoOpened] = useState(false);
  const [insertRuleOpened, setInsertRuleOpened] = useState(false);
  const [path, setPath] = useState("");
  const [search, setSearch] = useState("");
  useEffect(() => {
    flowResult(unrecognizedVideosStore.init());
  }, []);
  // useEffect(() => {
  //   unrecognizedVideosStore.update();
  // const unlisten = async () => {
  //   await listen<unrecognizedVideoObject[]>(
  //     "unrecognized_videos_list",
  //     (event) => {
  //       unrecognizedVideosStore.set_data(event.payload);
  //     }
  //   );
  // };
  //   return () => {
  //     unlisten();
  //   };
  // }, []);
  const form = useForm({
    initialValues: {
      type: "movie",
      path: "",
      id: "",
      provider: settingStore.setting.scraper.default_provider,
      lang: settingStore.setting.scraper.default_lang,
      title: "",
      season: 1,
      episode: 1,
    },
    validate: {
      type: (value) => {
        if (value !== "movie" && value !== "tvshow") {
          return t("unrecognized_videos.video_info.type_required");
        }
      },
      id: (value) => {
        if (!value) {
          return t("unrecognized_videos.video_info.id_required");
        }
      },
      provider: (value) => {
        if (!value) {
          return t("unrecognized_videos.video_info.provider_required");
        }
      },
      lang: (value) => {
        if (!/^[a-z]{2}-[A-Z]{2}$/g.test(value)) {
          return t("unrecognized_videos.video_info.lang_invalid");
        }
      },
      season: (value) => {
        if (value < 0 && form.values.type === "tvshow") {
          return t("unrecognized_videos.video_info.season_invalid");
        }
        if (value === undefined && form.values.type === "tvshow") {
          return t("unrecognized_videos.video_info.season_required");
        }
        if (value === undefined && form.values.type === "movie") {
          form.setFieldValue("season", 1);
        }
      },
      episode: (value) => {
        if (value < 0 && form.values.type === "tvshow") {
          return t("unrecognized_videos.video_info.episode_invalid");
        }
        if (value === undefined && form.values.type === "tvshow") {
          return t("unrecognized_videos.video_info.episode_required");
        }
        if (value === undefined && form.values.type === "movie") {
          form.setFieldValue("episode", 1);
        }
      },
    },
  });

  const theme = useMantineTheme();

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = event.currentTarget;
    setSearch(value);
  };

  const getRelativePath = (path: string) => {
    const root = settingStore.setting.storage.pending_path;
    if (path.startsWith(root)) {
      path = path.slice(root.length);
      if (path[0] === "/" || path[0] === "\\") {
        path = path.slice(1);
      }
    }
    return path;
  };

  const data = unrecognizedVideosStore.getUnrecognizedVideos
    .filter((item) => {
      if (search === "") {
        return true;
      }
      return getRelativePath(item?.path).includes(search);
    })
    .map((item) => (
      <tr key={item?.path}>
        <td>
          <Text
            size="sm"
            style={{ wordBreak: "break-all", wordWrap: "break-word" }}
          >
            {getRelativePath(item?.path)}
          </Text>
        </td>
        <td>
          <Group spacing={0} position="right">
            <ActionIcon
              onClick={() => {
                setPath(item?.path);
                setInsertRuleOpened(true);
              }}
            >
              <IconHeartPlus size={16} stroke={1.5} />
            </ActionIcon>
            <ActionIcon
              onClick={() => {
                form.setValues(item);
                setEditVideoOpened(true);
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
                      flowResult(unrecognizedVideosStore.delete(item?.path))
                        .then(() => {
                          notifications.show({
                            icon: <IconCheck />,
                            title: t("unrecognized_videos.delete_success"),
                            message: "✌️🙄✌️",
                          });
                        })
                        .catch((e) => {
                          notifications.show({
                            color: "red",
                            icon: <IconX />,
                            autoClose: false,
                            title: t("unrecognized_videos.delete_failed"),
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
        placeholder={t("unrecognized_videos.search_by_file_name") as string}
        mb="xs"
        onChange={handleSearchChange}
        icon={<IconSearch size={14} stroke={1.5} />}
      />
      <ScrollArea style={{ height: "calc(100vh - 110px)" }} type="scroll">
        <Table verticalSpacing="sm" striped highlightOnHover>
          <thead>
            <tr>
              <th>{t("unrecognized_videos.file_name")}</th>
              <th style={{ maxWidth: 100 }} />
            </tr>
          </thead>
          <tbody>{data}</tbody>
        </Table>
      </ScrollArea>
      <EditVideo
        opened={editVideoOpened}
        setOpened={setEditVideoOpened}
        form={form}
      />
      <InsertRule
        opened={insertRuleOpened}
        setOpened={setInsertRuleOpened}
        path={path}
      />
      <Affix
        hidden={settingStore.menu_open}
        position={{ bottom: 20, right: 20 }}
      >
        <ActionIcon
          size="xl"
          radius="xl"
          variant="filled"
          color={theme.primaryColor}
          onClick={() => fetch("refresh_unrecognized_videos_list")}
        >
          <IconRefresh stroke={1.5} size={34} />
        </ActionIcon>
      </Affix>
    </div>
  );
}

export default observer(UnrecognizedVideos);
