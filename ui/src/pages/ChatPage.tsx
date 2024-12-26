import {Sidebar} from "../components/Sidebar.tsx";
import {Chat} from "../components/Chat.tsx";
import {SettingsDialog} from "../components/SettingsDialog.tsx";

export function ChatPage() {
    return <>
        <Sidebar/>
        <Chat/>
        <SettingsDialog/>
    </>;
}