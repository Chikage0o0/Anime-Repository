import Layout from "@/layout";
import { useStore } from "@/store";
import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { emit } from "@tauri-apps/api/event";
import i18n from "i18next";
import { observer } from "mobx-react-lite";
import { useEffect } from "react";
import { initReactI18next } from "react-i18next";
import { BrowserRouter } from "react-router-dom";
import { resources } from "./locales/locales";
import settingStore from "./store/settingStore";

i18n
  // 将 i18n 实例传递给 react-i18next
  .use(initReactI18next)
  // 初始化 i18next
  // 所有配置选项: https://www.i18next.com/overview/configuration-options
  .init({
    resources,
    fallbackLng: "en_US",
    lng: settingStore.setting["ui"]["lang"],
    debug: false,
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    },
  });

function App() {
  // const outlet = useRoutes(routes)
  // const theme = getTheme(setting.setting as SettingObject)
  const { settingStore } = useStore();

  useEffect(() => {
    emit("show_window", {
      theMessage: "Hello World!",
    });
  }, []);

  return (
    <MantineProvider
      withGlobalStyles
      withNormalizeCSS
      theme={{
        colorScheme: settingStore.getColorScheme,
        primaryColor: settingStore.getPrimaryColor,
        primaryShade: { light: 5, dark: 8 },
        globalStyles: () => ({
          body: {
            WebkitUserSelect: "none",
            userSelect: "none",
          },
        }),
      }}
    >
      <BrowserRouter>
        <Notifications />
        <Layout />
      </BrowserRouter>
    </MantineProvider>
  );
}
export default observer(App);
