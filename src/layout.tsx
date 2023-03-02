import {
  AppShell,
  MediaQuery,
  Burger,
  useMantineTheme,
  Affix,
} from "@mantine/core";
import { Menu } from "@/components/menu";
import { useRoutes } from "react-router-dom";
import routes from "./router";
import { useStore } from "./store";
import { observer } from "mobx-react-lite";

function WrapperRoutes() {
  let element = useRoutes(routes);
  return element;
}

function Layout() {
  const theme = useMantineTheme();
  const { settingStore } = useStore();

  return (
    <AppShell navbarOffsetBreakpoint="sm" padding={0} navbar={<Menu />}>
      <Affix position={{ top: 20, right: 20 }}>
        <MediaQuery largerThan="sm" styles={{ display: "none" }}>
          <Burger
            opened={settingStore.menu_open}
            onClick={() => settingStore.setMenuOpen(!settingStore.menu_open)}
            size="sm"
            color={theme.colors.gray[6]}
            mr="xl"
          />
        </MediaQuery>
      </Affix>
      {/* 拖拽栏 */}
      <Affix position={{ top: 0, right: 0 }}>
        <div
          data-tauri-drag-region
          style={{
            height: "15px",
            display: "flex",
            justifyContent: "flex-end",
            position: "fixed",
            top: "0",
            right: "0",
            left: "0",
          }}
        />
      </Affix>
      <WrapperRoutes />
    </AppShell>
  );
}

export default observer(Layout);
