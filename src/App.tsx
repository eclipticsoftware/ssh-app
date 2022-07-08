import {createThemeGlobalStyles} from "@eclo/mode";
import {ThemeProvider} from "styled-components";
import {ConnectForm} from "./components/Screens/Connect.form";
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
      <ConnectForm />
    </ThemeProvider>
  );
}

export default App;
