import React from "react";
import {Dialog as DDialog, DialogPanel} from '@headlessui/react'
import {X} from "react-bootstrap-icons";

interface ButtonProps extends React.PropsWithChildren {
    className?: string;
}

export function Button({children, className}: ButtonProps) {
    return <button type="submit"
                   className={`w-full focus:ring-4 focus:outline-none font-medium rounded-lg text-sm px-5 py-2.5 text-center ${className}`}>
        {children}
    </button>;
}

export function Actions({children}: React.PropsWithChildren) {
    return <div className="flex flex-row gap-4 justify-between">
        {children}
    </div>;
}

function Title({children, onClose}: { onClose: () => void; } & React.PropsWithChildren) {
    return <div
        className="flex items-center justify-between p-4 border-b rounded-t dark:border-gray-600">
        <h1 className="text-l text-white">
            {children}
        </h1>
        <button type="button"
                onClick={() => onClose()}
                className="end-2.5 text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white outline-none"
        >
            <X className="size-8"/>
        </button>
    </div>
}

export interface DialogProps extends React.PropsWithChildren {
    open: boolean;
    onClose: () => void;
    title?: string
}

export function Dialog({open, children, onClose, title}: DialogProps) {
    return <DDialog open={open} onClose={() => onClose()} className="relative z-50">
        <div className="fixed inset-0 flex w-screen items-center justify-center p-4 bg-black bg-opacity-80">
            <DialogPanel className="relative bg-white rounded-lg shadow dark:bg-gray-700">
                {title && <Title onClose={() => onClose()}>{title}</Title>}
                <div className="p-4 md:p-5">
                    {children}
                </div>
            </DialogPanel>
        </div>
    </DDialog>
}