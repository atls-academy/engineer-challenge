import { createPromiseClient } from "@connectrpc/connect";
import { createConnectTransport } from "@connectrpc/connect-web";
import { AuthService } from "./gen/auth_connect";

const transport = createConnectTransport({
    baseUrl: "http://localhost:50051",
});

export const authClient = createPromiseClient(AuthService, transport);
