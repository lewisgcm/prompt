import * as MessageAction from "./MessageAction.tsx";
import * as React from "react";

export interface MessageProps extends React.PropsWithChildren {
    role: 'user' | 'assistant';
}

export function Message(props: MessageProps) {
    const rounded = props.role == 'user' ? 'rounded-br-none' : 'rounded-bl-none';
    const justify = props.role == 'user' ? 'flex-row-reverse' : 'flex-row';
    const color = props.role == 'user' ? 'bg-gray-400' : 'bg-gray-200';

    return <div className={`flex mt-4 ${justify}`}>
        <MessageAction.Dropdown>
            <MessageAction.DropdownItem onClick={() => {
            }}>
                Save
            </MessageAction.DropdownItem>
            <MessageAction.DropdownItem onClick={() => {
            }}>
                Copy
            </MessageAction.DropdownItem>
            <MessageAction.DropdownItem onClick={() => {
            }}>
                Delete
            </MessageAction.DropdownItem>
        </MessageAction.Dropdown>
        <div className={`border border-gray-400 rounded p-4 ${rounded} ${color}`}>
            {props.children}
        </div>
    </div>;
}