import { useStore } from "@/store";
import {
  Button,
  Center,
  Divider,
  Group,
  Modal,
  NumberInput,
  Popover,
  Select,
  Text,
  TextInput,
} from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import { IconCheck, IconSearch, IconX } from "@tabler/icons-react";
import { flowResult } from "mobx";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

function EditRule({
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
      title={t("subscribe_rules")}
    >
      <Divider mb="xs" size="xs" />
      <Group position="center" mb="xs" grow>
        <TextInput
          autoComplete="off"
          label={t("subscribe_rules.ID")}
          {...form.getInputProps("id")}
          rightSection={
            <IconSearch
              size={14}
              onClick={() => {
                let provider = form.values.provider;
                let id = form.values.id;
                if (provider === "tmdb") {
                  if (id !== "") {
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
          label={t("subscribe_rules.provider")}
          data={[{ value: "tmdb", label: "TMDB" }]}
          {...form.getInputProps("provider")}
        />
      </Group>
      <TextInput
        autoComplete="off"
        mb="xs"
        label={t("subscribe_rules.tvshow_regex")}
        {...form.getInputProps("tvshow_regex")}
      />
      <Group position="center" mb="xs" grow>
        <NumberInput
          autoComplete="off"
          label={t("subscribe_rules.season")}
          {...form.getInputProps("season")}
        />
        <TextInput
          autoComplete="off"
          label={t("subscribe_rules.lang")}
          placeholder="zh-CN, en-US, etc"
          {...form.getInputProps("lang")}
        />
      </Group>
      <Group position="center" mb="xs" grow>
        <TextInput
          autoComplete="off"
          label={t("subscribe_rules.episode_regex")}
          placeholder="\\d+"
          {...form.getInputProps("episode_regex")}
        />
        <NumberInput
          autoComplete="off"
          label={t("subscribe_rules.episode_position")}
          {...form.getInputProps("episode_position")}
        />
        <NumberInput
          autoComplete="off"
          label={t("subscribe_rules.episode_offset")}
          {...form.getInputProps("episode_offset")}
        />
      </Group>
      <Divider mb="sm" size="xs" />
      <Group position="center" grow>
        <Button variant="outline" color="red" onClick={() => form.reset()}>
          {t("UI.reset")}
        </Button>
        <Submit form={form} setOpened={setOpened} />
      </Group>
    </Modal>
  );
}

export default EditRule;

function Submit({
  form,
  setOpened,
}: {
  form: UseFormReturnType<any, any>;
  setOpened: Dispatch<SetStateAction<boolean>>;
}) {
  const { subscribeRulesStore } = useStore();
  const { t } = useTranslation();
  const [confirmOpened, setConfirmOpened] = useState(false);
  const getTitle = async (id: string, provider: string, lang: string) => {
    if (id && provider && lang) {
      let result = await fetch(
        `api/get_title/${id}/${provider}/${lang}/tvshow`
      );
      if (result.ok) {
        let title = await result.json();
        return title;
      } else {
        const e: string = await result.text();
        throw result.statusText + "\n" + e;
      }
    }
  };

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
                  form.values.lang
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
                  message: error,
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
                flowResult(subscribeRulesStore.addSubscribeRule(form.values))
                  .then(() => {
                    form.reset();
                    setOpened(false);
                    notifications.show({
                      icon: <IconCheck />,
                      title: t("subscribe_rules.insert_success"),
                      message: "✌️🙄✌️",
                    });
                  })
                  .catch((e) => {
                    notifications.show({
                      color: "red",
                      icon: <IconX />,
                      autoClose: false,
                      title: t("subscribe_rules.insert_failed"),
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
