interface ChatListProps {
    chats: {
        name: string;
    }[];
}

export function ChatList({chats}: ChatListProps) {
    return <div className={`border-r border-solid border-black flex-grow p-4 w-64 min-w-64 max-w-64`}>
        <div className="font-bold">
            Active chats
        </div>
        {chats.map((chat) => {
            return <button key={chat.name}
                           className="flex flex-col border border-gray-500 w-full mb-1 rounded p-1 select-none hover:bg-gray-200">
                <div className="relative max-w-full">
                    <div className="text-left whitespace-nowrap overflow-ellipsis overflow-hidden">
                        {chat.name}
                    </div>
                </div>
                <div className="relative max-w-full">
                    <div className="text-left whitespace-nowrap overflow-ellipsis overflow-hidden">
                        Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been
                    </div>
                </div>
                <div className="text-right text-xs w-full">
                    Yesterday
                </div>
            </button>
        })}
    </div>;
}