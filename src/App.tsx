import {createThemeGlobalStyles} from "@eclo/mode";
import {ThemeProvider} from "styled-components";
import {MainScreen} from "./components/Screens/Main.screen";
import {GlobalStyles} from "./theme/globalStyles";
import {theme} from "./theme/theme";

const App = () => {

	const globalStyles = createThemeGlobalStyles(theme, {
		include: {
			cssReset: true,
			fluidHeadings: true,
		},
	})

  return (
    <ThemeProvider theme={theme}>
      <GlobalStyles globalStyles={globalStyles} />
      <MainScreen />
    </ThemeProvider>
  );
}

export default App;
