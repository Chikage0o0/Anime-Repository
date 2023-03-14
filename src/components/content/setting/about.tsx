import {
  createStyles,
  Avatar,
  Text,
  Group,
  Anchor,
  Divider,
  Flex,
  Stack,
} from "@mantine/core";
import {
  IconBrandGithub,
  IconHomeLink,
  IconSignRight,
} from "@tabler/icons-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

const useStyles = createStyles((theme) => ({
  icon: {
    color:
      theme.colorScheme === "dark"
        ? theme.colors.dark[3]
        : theme.colors.gray[5],
  },

  name: {
    fontFamily: `Greycliff CF, ${theme.fontFamily}`,
  },
}));

interface DeveloperProps {
  avatar: string;
  name: string;
  title: string;
  home_page: string;
  home_page_name: string;
  sign: string;
}

const developer_list: DeveloperProps[] = [
  {
    avatar: "https://avatars.githubusercontent.com/u/89348590",
    name: "Chikage",
    title: "Developer",
    home_page: "https://github.com/Chikage0o0",
    sign: "我永远喜欢蕾西亚",
    home_page_name: "GitHub",
  },
];

export default function About({ classes }: { classes: any }) {
  const developer = developer_list.map((item) => {
    return <Developer key={item.name} {...item} />;
  });
  const { t } = useTranslation();

  return (
    <>
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
          Version: 0.2.0
        </Text>
      </Stack>
    </>
  );
}

function Developer({
  avatar,
  name,
  title,
  home_page_name,
  home_page,
  sign,
}: DeveloperProps) {
  const { classes } = useStyles();
  return (
    <Group noWrap>
      <Avatar src={avatar} size={94} radius="md" />
      <div>
        <Text fz="xs" tt="uppercase" fw={700} c="dimmed">
          {title}
        </Text>

        <Text fz="lg" fw={500} className={classes.name}>
          {name}
        </Text>

        <Group noWrap spacing={10} mt={3}>
          <IconSignRight stroke={1.5} size="1rem" className={classes.icon} />
          <Text fz="xs" c="dimmed">
            {sign}
          </Text>
        </Group>

        <Group noWrap spacing={10} mt={5}>
          <IconHomeLink stroke={1.5} size="1rem" className={classes.icon} />
          <Anchor
            onClick={() => {
              window.open(home_page, "_blank");
            }}
            fz="xs"
            c="dimmed"
          >
            {home_page_name}
          </Anchor>
        </Group>
      </div>
    </Group>
  );
}
