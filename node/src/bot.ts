import { Bot } from 'grammy';

const BOT_TOKEN = process.env.BOT_TOKEN;
if (!BOT_TOKEN) {
	console.error('BOT_TOKEN environment variable is not set');
	process.exit(1);
}

const BOT_NAME = process.env.BOT_NAME;
if (!BOT_NAME) {
	console.error('BOT_NAME environment variable is not set');
	process.exit(1);
}

const BOT_INVITATION = `@${BOT_NAME} `;

function getRandomInt(min: number, max: number): number {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

const roll = (msg: string): string => {
	const [diceCount, facesCount] = msg.toLowerCase().trim().split('d').map((val) => Math.abs((Number(val) | 0)) || 1);
	const results = Array<number>(diceCount).fill(0).map(() => getRandomInt(1, facesCount));

	if (results.length === 1) {
		return results[0].toString();
	}

	const sum = results.reduce((sum, next) => sum + next, 0);
	return `${results.join(' + ')} = ${sum}`;
};

const bot = new Bot(BOT_TOKEN);

bot.on('message:text', (ctx) => {

	const {message: {chat: {type: chatType}, from: {username: fromUserName}, text}} = ctx;

	switch (chatType) {
		case 'private':
			ctx.reply(roll(text));
			break;

		case 'group':
		case 'supergroup':
			if (text.startsWith(BOT_INVITATION) && fromUserName) {
				const command = text.slice(BOT_INVITATION.length);
				const rollResult = roll(command);
				ctx.reply(`@${fromUserName} ${command}\n${rollResult}`);
			}
			break;

		default:
			break;
	}
});

bot.start({
	onStart(botInfo) {
		console.info(`${botInfo.username} successfully started`);
	},
});
