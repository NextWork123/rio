"use strict";(self.webpackChunkrio_docs=self.webpackChunkrio_docs||[]).push([[6557],{3905:(t,e,a)=>{a.d(e,{Zo:()=>p,kt:()=>c});var n=a(7294);function r(t,e,a){return e in t?Object.defineProperty(t,e,{value:a,enumerable:!0,configurable:!0,writable:!0}):t[e]=a,t}function i(t,e){var a=Object.keys(t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(t);e&&(n=n.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),a.push.apply(a,n)}return a}function l(t){for(var e=1;e<arguments.length;e++){var a=null!=arguments[e]?arguments[e]:{};e%2?i(Object(a),!0).forEach((function(e){r(t,e,a[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(a)):i(Object(a)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(a,e))}))}return t}function d(t,e){if(null==t)return{};var a,n,r=function(t,e){if(null==t)return{};var a,n,r={},i=Object.keys(t);for(n=0;n<i.length;n++)a=i[n],e.indexOf(a)>=0||(r[a]=t[a]);return r}(t,e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(t);for(n=0;n<i.length;n++)a=i[n],e.indexOf(a)>=0||Object.prototype.propertyIsEnumerable.call(t,a)&&(r[a]=t[a])}return r}var o=n.createContext({}),m=function(t){var e=n.useContext(o),a=e;return t&&(a="function"==typeof t?t(e):l(l({},e),t)),a},p=function(t){var e=m(t.components);return n.createElement(o.Provider,{value:e},t.children)},k="mdxType",N={inlineCode:"code",wrapper:function(t){var e=t.children;return n.createElement(n.Fragment,{},e)}},f=n.forwardRef((function(t,e){var a=t.components,r=t.mdxType,i=t.originalType,o=t.parentName,p=d(t,["components","mdxType","originalType","parentName"]),k=m(a),f=r,c=k["".concat(o,".").concat(f)]||k[f]||N[f]||i;return a?n.createElement(c,l(l({ref:e},p),{},{components:a})):n.createElement(c,l({ref:e},p))}));function c(t,e){var a=arguments,r=e&&e.mdxType;if("string"==typeof t||r){var i=a.length,l=new Array(i);l[0]=f;var d={};for(var o in e)hasOwnProperty.call(e,o)&&(d[o]=e[o]);d.originalType=t,d[k]="string"==typeof t?t:r,l[1]=d;for(var m=2;m<i;m++)l[m]=a[m];return n.createElement.apply(null,l)}return n.createElement.apply(null,a)}f.displayName="MDXCreateElement"},1987:(t,e,a)=>{a.r(e),a.d(e,{assets:()=>o,contentTitle:()=>l,default:()=>N,frontMatter:()=>i,metadata:()=>d,toc:()=>m});var n=a(7462),r=(a(7294),a(3905));const i={title:"Alacritty's Vi mode",language:"en"},l=void 0,d={unversionedId:"features/alacritty-vi-mode",id:"features/alacritty-vi-mode",title:"Alacritty's Vi mode",description:"Rio implements Alacritty's Vi mode.",source:"@site/docs/features/alacritty-vi-mode.md",sourceDirName:"features",slug:"/features/alacritty-vi-mode",permalink:"/rio/docs/features/alacritty-vi-mode",draft:!1,editUrl:"https://github.com/raphamorim/rio/tree/main/docs/docs/features/alacritty-vi-mode.md",tags:[],version:"current",frontMatter:{title:"Alacritty's Vi mode",language:"en"},sidebar:"tutorialSidebar",previous:{title:"Adaptive theme",permalink:"/rio/docs/features/adaptive-theme"},next:{title:"Color automation for navigation",permalink:"/rio/docs/features/color-automation-for-navigation"}},o={},m=[],p={toc:m},k="wrapper";function N(t){let{components:e,...i}=t;return(0,r.kt)(k,(0,n.Z)({},p,i,{components:e,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"Rio implements ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/alacritty/alacritty/blob/master/docs/features.md#vi-mode"},"Alacritty's Vi mode"),"."),(0,r.kt)("p",null,"By default you can launch Vi mode by using ",(0,r.kt)("inlineCode",{parentName:"p"},"alt")," + ",(0,r.kt)("inlineCode",{parentName:"p"},"shift")," + ",(0,r.kt)("inlineCode",{parentName:"p"},"space"),"."),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Demo alacritty&#39;s Vi mode",src:a(4744).Z,width:"640",height:"462"})),(0,r.kt)("p",null,"Below you can see the list of all default key bindings related to Vi mode. If you don't like of any specified key binding you can always turn off or modify (check ",(0,r.kt)("a",{parentName:"p",href:"/rio/docs/documentation/key-bindings"},"key bindings documentation section")," for more information)."),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:"left"},"Trigger"),(0,r.kt)("th",{parentName:"tr",align:"left"},"Action"),(0,r.kt)("th",{parentName:"tr",align:"left"},"Condition"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"alt")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"space")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Toggle Vi Mode"),(0,r.kt)("td",{parentName:"tr",align:"left"},"No restriction")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"i")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Toggle Vi Mode"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"control")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"c")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Toggle Vi Mode"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"y")),(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"Copy")," or ",(0,r.kt)("inlineCode",{parentName:"td"},"ClearSelection")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"v")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Start normal selection"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"v")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Start line selection"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"v")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Start block selection"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"v")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"alt")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Start semantic selection"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"z")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Center around Vi cursor"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"y")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll up 1 line"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"e")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll down 1 line"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"b")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll page up"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"u")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll half page up"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"d")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll half page down"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"e")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"control")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Scroll down 1 line"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"k")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor up"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"j")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor down"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"h")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor left"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"l")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},"Arrow up"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor up"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},"Arrow down"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor down"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},"Arrow left"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor left"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},"Arrow right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},"Arrow right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move cursor right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"0")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move first"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"4")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move last"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"6")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move first occupied"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"h")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move high"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"m")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move middle"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"l")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move low"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"b")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move semantic left"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"w")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move semantic right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"e")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move semantic right end"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"b")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move word left"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"w")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move word right"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"e")," + ",(0,r.kt)("inlineCode",{parentName:"td"},"shift")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move word right end"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"left"},(0,r.kt)("inlineCode",{parentName:"td"},"5")),(0,r.kt)("td",{parentName:"tr",align:"left"},"Move by bracket rule"),(0,r.kt)("td",{parentName:"tr",align:"left"},"Vi mode is activated")))))}N.isMDXComponent=!0},4744:(t,e,a)=>{a.d(e,{Z:()=>n});const n=a.p+"assets/images/demo-alacritty-vi-mode-27a4264d4d1ffe0266336aafc2d40240.gif"}}]);