import { PUBLIC_APP_PLATFORM } from "$env/static/public";
export const ssr = PUBLIC_APP_PLATFORM == "web" ? false : true;
