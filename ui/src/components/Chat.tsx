import {useDispatch, useSelector} from "react-redux";
import {BeatLoader} from "react-spinners";
import {GearWideConnected, LayoutSidebar, Paperclip, Send, Trash3} from "react-bootstrap-icons";

import {Message} from "./Message.tsx";
import * as TopBar from './TopBar';
import {RootState} from "../store.ts";
import {open} from "../state/sidebar.ts";
import {open as openSettings} from "../state/settings.ts";

function AttachmentButton() {
    return <button className="h-8 w-8 mr-4 align-middle border hover:border-black border-transparent rounded p-1">
        <Paperclip className="size-full"/>
    </button>;
}

function SendButton() {
    return <button className="h-8 w-8 align-middle border hover:border-black border-transparent rounded p-1">
        <Send className="size-full"/>
    </button>;
}

export function Chat() {
    const isOpen = useSelector((state: RootState) => state.sidebar.value);
    const dispatch = useDispatch();

    return <div className="grow flex-col flex">
        <TopBar.Container>
            {!isOpen && <TopBar.Button onClick={() => dispatch(open())}>
                <LayoutSidebar className="size-full"/>
            </TopBar.Button>}
            <span className="flex flex-row items-center ml-4">
                <p>WBR 2024</p>
            </span>
            <span className="flex-grow"/>
            <TopBar.Button>
                <Trash3 className="size-full"/>
            </TopBar.Button>
            <TopBar.Button onClick={() => dispatch(openSettings())}>
                <GearWideConnected className="size-full"/>
            </TopBar.Button>
        </TopBar.Container>
        <div className="grow p-4 overflow-y-scroll flex flex-col-reverse bg-gray-500">
            <Message role='assistant'>
                <div>
                    <BeatLoader size={5} cssOverride={{display: 'inline-block', marginRight: '0.5em'}}/>
                    Waiting for a response.
                </div>
            </Message>
            <Message role='assistant'>
                Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the
                industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and
                scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap
                into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the
                release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing
                software like Aldus PageMaker including versions of Lorem Ipsum.
            </Message>
            <Message role='user'>
                What is lorem ipsum?
            </Message>
        </div>
        <div className="flex p-4 border-t border-black">
            <AttachmentButton/>
            <input type="text" className="border border-black rounded-full grow mr-4 pl-2"/>
            <SendButton/>
        </div>
    </div>;
}