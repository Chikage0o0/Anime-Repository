import { createStyles, Avatar, Text, Group, Anchor } from "@mantine/core";
import { IconHomeLink, IconSignRight } from "@tabler/icons-react";

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

export const developer_list: DeveloperProps[] = [
  {
    avatar: "https://avatars.githubusercontent.com/u/89348590",
    name: "Chikage",
    title: "Developer",
    home_page: "https://github.com/Chikage0o0",
    sign: "我永远喜欢蕾西亚",
    home_page_name: "GitHub",
  },
];

export function Developer({
  avatar,
  name,
  title,
  home_page_name,
  home_page,
  sign,
}: DeveloperProps) {
  const { classes } = useStyles();
  return (
    <div>
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
    </div>
  );
}
