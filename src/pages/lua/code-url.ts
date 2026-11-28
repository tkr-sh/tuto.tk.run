// Encode/decode Lua source to a URL-safe base64 string so that code can travel
// in the editor URL (`#code=...`) without ever hitting the server.

const bytesToBinary = (bytes: Uint8Array): string => {
    let binary = "";
    for (let i = 0; i < bytes.length; i++) binary += String.fromCharCode(bytes[i]);
    return binary;
};

const encodeCodeToUrl = (code: string): string =>
    btoa(bytesToBinary(new TextEncoder().encode(code)))
        .replace(/\+/g, "-")
        .replace(/\//g, "_")
        .replace(/=+$/, "");

const decodeCodeFromUrl = (value: string): string => {
    const base64 = value.replace(/-/g, "+").replace(/_/g, "/");
    const padding = (4 - (base64.length % 4)) % 4;
    const binary = atob(base64 + "=".repeat(padding));
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
    return new TextDecoder().decode(bytes);
};
