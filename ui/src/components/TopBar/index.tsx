import * as React from "react";

interface ButtonsProps extends React.PropsWithChildren {
    className?: string;
    onClick?: () => void;
}

export function Button({children, className, onClick}: ButtonsProps) {
    return <button type="button"
                   onClick={onClick}
                   className={`hover:bg-gray-300 p-1 w-8 h-8 m-2 rounded ${className || ''}`}>
        {children}
    </button>;
}

interface ContainerProps extends React.PropsWithChildren {
}

export function Container({children}: ContainerProps) {
    return <div className="flex border-b border-black bg-gray-200">
        {children}
    </div>;
}

interface DividerProps {
    className?: string;
}

export function Divider({className}: DividerProps) {
    return <div className={`border-opacity-40 border-r border-black mt-1 mb-1 ${className || ''}`}>
    </div>;
}