import { createGlobalStyle } from 'styled-components'
import { reactModalStyles } from './reactModal.styles'

export const GlobalStyles = createGlobalStyle<{ globalStyles: any; theme: any }>`

${reactModalStyles}

${props => (props.globalStyles ? props.globalStyles : '')}

//css reset
html, body, div, span, applet, object, iframe,
h1, h2, h3, h4, h5, h6, p, blockquote, pre,
a, abbr, acronym, address, big, cite, code,
del, dfn, em, img, ins, kbd, q, s, samp,
small, strike, strong, sub, sup, tt, var,
b, u, i, center,
dl, dt, dd, ol, ul, li,
fieldset, form, label, legend,
table, caption, tbody, tfoot, thead, tr, th, td,
article, aside, canvas, details, embed, 
figure, figcaption, footer, header, hgroup, 
menu, nav, output, ruby, section, summary,
time, mark, audio, video {
	margin: 0;
	padding: 0;
	border: 0;
	font-size: 100%;
	font: inherit;
	vertical-align: baseline;
}
/* HTML5 display-role reset for older browsers */
article, aside, details, figcaption, figure, 
footer, header, hgroup, menu, nav, section {
	display: block;
}
body {
	line-height: 1;
  overflow-x: hidden ;
}
ol, ul {
	list-style: none;
}
blockquote, q {
	quotes: none;
}
blockquote:before, blockquote:after,
q:before, q:after {
	content: '';
	content: none;
}
table {
	border-collapse: collapse;
	border-spacing: 0;
}

html {
    scroll-behavior: smooth;
  }

  body {
    padding: 0;
    margin: 0;
    font-family: ${props => props.theme.fonts.textFamily};
    font-size: ${p => p.theme.sizes.baseFont};
    position:relative;
    width: 100%;
    color: ${props => props.theme.colors.black.val};
    &.ReactModal__Body--open {
      overflow: hidden;
    }
  }
  h1,h2,h3,h4,h5,h6 {
    font-family: ${props => props.theme.fonts.titleFamily};
  }
  div, span, applet, object, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, a, abbr, acronym, address, big, cite, code, del, dfn, em, img, ins, kbd, q, s, samp, small, strike, strong, sub, sup, tt, var, b, u, i, center, dl, dt, dd, ol, ul, li, fieldset, form, label, legend, table, caption, tbody, tfoot, thead, tr, th, td, article, aside, canvas, details, embed, figure, figcaption, footer, header, hgroup, main, menu, nav, output, ruby, section, summary, time, mark, audio, video {
    font-size: inherit;
  }
  div, p, main, aside, header, footer, blockquote, table, ul, section, legend, input, textarea, select, button, a {
    position:relative;
    box-sizing: border-box;
  }
  p, span, blockquote, li, a, em, i {
    line-height: 1.3;
  }
  strong {
    font-weight: 600;
  }
  h3,h5 {
    font-weight: 600;
  }
  h2,h4 {
    font-weight: 400;
  }

  .rt-content {
    h1,h2,h3,h4,h5,h6,p {
      margin: 1em 0;
    }
    ul {
      margin-left: 2em;
      list-style: circle;
    }
    a {
      overflow-wrap: break-word;
    }
  }
  .flex-col {
    flex-grow: 1;
  }
  .grid {
    display: grid;
  }
  .overlay-bg {
    top:0;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 5;
    position: absolute;
    background: ${props => props.theme.colors.white.opacity().val};
    transition: opacity ${props => props.theme.times.tranS};
  }
  .flex {
    display: flex;
  }
  .flex-tablet {
    ${props => props.theme.media.tablet} {
      display: flex;
      &.stretch {
        justify-content: stretch;
      }
    }
  }
  .flex-desk {
    ${props => props.theme.media.sdesk} {
      display: flex;
      &.stretch {
        justify-content: stretch;
      }
    }
  }
`
