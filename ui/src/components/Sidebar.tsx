import {useSelector, useDispatch} from 'react-redux'
import {LayoutSidebar, PencilSquare} from "react-bootstrap-icons";

import * as TopBar from './TopBar';
import {RootState} from "../store.ts";
import {close} from '../state/sidebar.ts'
import {ChatList} from "./ChatList.tsx";

export function Sidebar() {
    const isOpen = useSelector((state: RootState) => state.sidebar.value);
    const dispatch = useDispatch();

    const chats = [
        {name: 'WBR 2024'},
        {name: 'QBR 2024'},
    ];

    return <aside
        className={`relative flex flex-col h-screen w-64 min-w-64 max-w-64 overflow-hidden transition-translate transition-width duration-300 ${!isOpen && '-translate-x-full !w-0 !min-w-0'}`}>
        <TopBar.Container>
            <TopBar.Button onClick={() => dispatch(close())}>
                <LayoutSidebar className="size-full"/>
            </TopBar.Button>
            <div className="flex-grow"/>
            <TopBar.Button>
                <PencilSquare className="size-full"/>
            </TopBar.Button>
            <TopBar.Divider/>
        </TopBar.Container>
        <ChatList chats={chats}/>
    </aside>
}