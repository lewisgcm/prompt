package internal

type TextInputMessage struct {
	Text string
}

type ImageInputMessage struct {
	Format string
	Image  []uint8
}

type DocumentInputMessage struct {
	Format   string
	Document []uint8
}

type Message struct {
	Role string
}

type MessageContentBlock struct {
	Role string
}
