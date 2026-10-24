import axios, { AxiosError } from "axios";
import { useRetrac } from "src/wrapper/retrac";
import { useApplicationInformation } from "src/wrapper/tauri";

export const protocol = ((): string => {
  return "http";
})();

export const hostname = ((): string => {
  if (useRetrac.getState().override_client_url != "") {
    return useRetrac.getState().override_client_url.split("://")[1];
  }
  return useApplicationInformation.getState().dev
    ? "26.87.28.199:3551"
    : "26.87.28.199:3551";
})();

export const axiosClient = axios.create({
  baseURL: `${protocol}://${hostname}`,
});

const get_discord_login_url = async (): Promise<RResponse<string>> => {
  return {
    ok: true,
    data: "https://discord.com/api/oauth2/authorize?client_id=1563227407702425730&redirect_uri=http%3A%2F%2F26.87.28.199%3A8080%2Foauth2%2Fcallback&response_type=code&scope=identify%20email",
  };
};

const get_lootlabs_offer_url = async (
  access_token: string
): Promise<RResponse<string>> => {
  const response = await axiosClient
    .get<string>("/adverts", {
      headers: {
        Authorization: access_token,
      },
    })
    .catch((e: AxiosError<any>) => {
      return e;
    });

  if (response instanceof AxiosError) {
    return {
      ok: false,
      error: response.message,
    };
  }

  return {
    ok: true,
    data: response.data,
  };
};

export default { get_discord_login_url, get_lootlabs_offer_url };