import { useStore } from "@/store";
import {
  Box,
  Button,
  Center,
  Group,
  Modal,
  NumberInput,
  SegmentedControl,
  Select,
  Text,
  TextInput,
} from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import {
  IconCheck,
  IconDeviceTv,
  IconMovie,
  IconSearch,
  IconX,
} from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api";
import { flowResult } from "mobx";
import { Dispatch, SetStateAction } from "react";
import { useTranslation } from "react-i18next";

function EditVideo({
  opened,
  setOpened,
  form,
}: {
  opened: boolean;
  setOpened: Dispatch<SetStateAction<boolean>>;
  form: UseFormReturnType<any, any>;
}) {
  const { unrecognizedVideosStore } = useStore();

  const { t } = useTranslation();
  return (
    <Modal
      size="lg"
      opened={opened}
      onClose={() => setOpened(false)}
      title={t("unrecognized_videos.video_info")}
    >
      <Text size="sm" mb="sm">
        {form.values["path"]}
      </Text>
      <SegmentedControl
        fullWidth
        mb="xs"
        data={[
          {
            value: "movie",
            label: (
              <Center>
                <IconMovie size={16} />
                <Box ml={10}>Movie</Box>
              </Center>
            ),
          },
          {
            value: "tvshow",
            label: (
              <Center>
                <IconDeviceTv size={16} />
                <Box ml={10}>Tvshow</Box>
              </Center>
            ),
          },
        ]}
        {...form.getInputProps("type")}
      />
      <Group position="center" mb="sm" grow>
        <TextInput
          autoComplete="off"
          label={t("unrecognized_videos.video_info.ID")}
          {...form.getInputProps("id")}
          rightSection={
            <IconSearch
              size={14}
              onClick={() => {
                let provider = form.values.provider;
                let id = form.values.id;
                if (provider === "tmdb") {
                  if (form.values.type === "movie") {
                    if (id !== "") {
                      window.open(
                        "https://www.themoviedb.org/movie/" + id,
                        "_blank"
                      );
                    } else
                      window.open(
                        "https://www.themoviedb.org/search/movie?query=",
                        "_blank"
                      );
                  } else if (id !== "") {
                    window.open(
                      "https://www.themoviedb.org/tv/" + id,
                      "_blank"
                    );
                  } else
                    window.open(
                      "https://www.themoviedb.org/search/tv?query=" +
                        form.values.title,
                      "_blank"
                    );
                }
              }}
            />
          }
        />
        <Select
          label={t("unrecognized_videos.video_info.provider")}
          data={[{ value: "tmdb", label: "TMDB" }]}
          {...form.getInputProps("provider")}
        />
        <TextInput
          autoComplete="off"
          label={t("unrecognized_videos.video_info.lang")}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps("lang")}
        />
      </Group>
      <Group
        position="center"
        mb="sm"
        grow
        hidden={form.values["type"] === "tvshow" ? false : true}
      >
        <NumberInput
          autoComplete="off"
          label={t("unrecognized_videos.video_info.season")}
          {...form.getInputProps("season")}
        />
        <NumberInput
          autoComplete="off"
          label={t("unrecognized_videos.video_info.episode")}
          {...form.getInputProps("episode")}
        />
      </Group>
      <Group
        position="center"
        mb="sm"
        grow
        hidden={form.values["type"] === "tvshow" ? false : true}
      >
        <TextInput
          autoComplete="off"
          mb="xs"
          label={t("unrecognized_videos.video_info.title")}
          onClick={() => {
            if (form.values.id && form.values.provider && form.values.lang) {
              invoke("get_tvshow_title", {
                id: form.values.id,
                provider: form.values.provider,
                lang: form.values.lang,
              })
                .then((res) => {
                  form.setFieldValue("title", res as string);
                })
                .catch((e) => {
                  notifications.show({
                    color: "red",
                    icon: <IconX />,
                    autoClose: false,
                    title: t("unrecognized_videos.video_info.title_not_found"),
                    message: e,
                  });
                });
            }
          }}
          {...form.getInputProps("title")}
        />
      </Group>
      <Group position="center" mt="xl" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t("UI.reset")}
        </Button>
        <Button
          variant="outline"
          color="blue"
          onClick={() => {
            if (!form.validate().hasErrors) {
              form.reset();
              setOpened(false);
              flowResult(unrecognizedVideosStore.submit(form.values))
                .then(() => {
                  notifications.show({
                    icon: <IconCheck />,
                    title: t("unrecognized_videos.video_info.submit_success"),
                    message: "✌️🙄✌️",
                  });
                })
                .catch((e) => {
                  notifications.show({
                    color: "red",
                    icon: <IconX />,
                    autoClose: false,
                    title: t("unrecognized_videos.video_info.submit_failed"),
                    message: e,
                  });
                });
            }
          }}
        >
          {t("UI.submit")}
        </Button>
      </Group>
    </Modal>
  );
}

export default EditVideo;
