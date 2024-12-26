import {Menu, MenuButton, MenuItem, MenuItems} from '@headlessui/react'
import React from "react";
import {ThreeDotsVertical} from "react-bootstrap-icons";

interface MessageActionProps extends React.PropsWithChildren {
}

interface DropdownItemProps extends React.PropsWithChildren {
    onClick: () => void;
}

export function DropdownItem({children, onClick}: DropdownItemProps) {
    return <MenuItem>
        <a className="select-none cursor-pointer block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
           onClick={onClick}>
            {children}
        </a>
    </MenuItem>;
}

export function Dropdown({children}: MessageActionProps) {
    return <div className="relative flex">
        <div className="m-auto mr-1 ml-1">
            <Menu>
                <MenuButton
                    className="group inline-flex w-6 h-6 justify-center gap-x-1.5 rounded-full px-1 py-1 text-sm font-semibold text-gray-300 hover:shadow-sm hover:bg-gray-400 focus:outline-none">
                    <ThreeDotsVertical className="size-full"/>
                </MenuButton>
                <MenuItems anchor='left'
                           className="z-10 mt-2 w-32 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none">
                    {children}
                </MenuItems>
            </Menu>
        </div>
    </div>

}