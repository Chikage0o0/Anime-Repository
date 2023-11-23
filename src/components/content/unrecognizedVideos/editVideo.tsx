import { getTitle } from "@/invoke";
import { useStore } from "@/store";
import {
  Box,
  Button,
  Center,
  Group,
  Modal,
  NumberInput,
  Popover,
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
import { flowResult } from "mobx";
import { Dispatch, SetStateAction, useState } from "react";
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
  const { t } = useTranslation();
  return (
    <Modal
      size="lg"
      opened={opened}
      onClose={() => setOpened(false)}
      centered
      title={t("unrecognized_videos.video_info")}
    >
      <Text
        size="sm"
        mb="sm"
        style={{ wordBreak: "break-all", wordWrap: "break-word" }}
      >
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
      <div hidden={!(form.values["type"] === "tvshow")}>
        <Group position="center" mb="sm" grow>
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
      </div>
      <Group position="center" mt="xl" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t("UI.reset")}
        </Button>
        <Submit form={form} setOpened={setOpened} />
      </Group>
    </Modal>
  );
}

export default EditVideo;

function Submit({
  form,
  setOpened,
}: {
  form: UseFormReturnType<any, any>;
  setOpened: Dispatch<SetStateAction<boolean>>;
}) {
  const { unrecognizedVideosStore } = useStore();
  const { t } = useTranslation();
  const [confirmOpened, setConfirmOpened] = useState(false);

  return (
    <Popover position="bottom" withArrow shadow="md" opened={confirmOpened}>
      <Popover.Target>
        <Button
          variant="outline"
          color="blue"
          onClick={async () => {
            if (!form.validate().hasErrors) {
              try {
                let title = await getTitle(
                  form.values.id,
                  form.values.provider,
                  form.values.lang,
                  form.values.type
                );
                if (title) {
                  form.setFieldValue("title", title);
                  setConfirmOpened(true);
                }
              } catch (error: any) {
                notifications.show({
                  color: "red",
                  icon: <IconX />,
                  autoClose: false,
                  title: t("UI.get_info_failed"),
                  message: error.message,
                });
              }
            }
          }}
        >
          {t("UI.submit")}
        </Button>
      </Popover.Target>
      <Popover.Dropdown>
        <Text size="md">{form.values.title}</Text>
        <Center>
          <Group>
            <Button
              variant="outline"
              radius="xs"
              size="sm"
              mt="xs"
              onClick={() => {
                setConfirmOpened(false);
              }}
              compact
            >
              {t("UI.false")}
            </Button>
            <Button
              variant="outline"
              radius="xs"
              size="sm"
              mt="xs"
              color="blue"
              compact
              onClick={() => {
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
              }}
            >
              {t("UI.true")}
            </Button>
          </Group>
        </Center>
      </Popover.Dropdown>
    </Popover>
  );
}
