// import { } from '@fortawesome/pro-duotone-svg-icons'
import { library } from '@fortawesome/fontawesome-svg-core'
import * as light from '@fortawesome/pro-light-svg-icons'
import get from 'lodash/get'
import values from 'lodash/values'

const iconLibAlerts = {
	err: ['fal', 'exclamation-circle'],
	ok: ['fal', 'check-circle'],
	warn: ['fal', 'exclamation'],
	info: ['fal', 'info-circle'],
	alert: ['fal', 'exclamation-triangle'],
	bellEmpty: ['fal', 'bell'],
	bellSlash: ['fal', 'bell-slash'],
	bellAlert: ['fal', 'bell-exclamation'],
	pending: ['fal', 'hourglass-half'],
	ssh: ['fal', 'thunderstorm'],
	circle: ['fal', 'circle'],
}

const iconLibDirectional = {
	arrowRight: ['fal', 'arrow-right'],
	arrowLeft: ['fal', 'arrow-left'],
	arrowDown: ['fal', 'arrow-down'],
	arrowUp: ['fal', 'arrow-up'],
	chevronRight: ['fal', 'chevron-right'],
	chevronLeft: ['fal', 'chevron-left'],
	chevronUp: ['fal', 'chevron-up'],
	chevronDown: ['fal', 'chevron-down'],
	caretRightEmpty: ['fal', 'caret-right'],
	caretLeftEmpty: ['fal', 'caret-left'],
	caretBoxRight: ['fal', 'caret-square-right'],
	caretBoxLeft: ['fal', 'caret-square-left'],
	caretBoxDown: ['fal', 'caret-square-down'],
	caretBoxUp: ['fal', 'caret-square-up'],
	forward: ['fal', 'forward'],
	back: ['fal', 'backward'],
	fastForward: ['fal', 'fast-forward'],
	fastBack: ['fal', 'fast-backward'],
	sortUp: ['fal', 'sort-amount-up'],
	sortDown: ['fal', 'sort-amount-down'],
	update: ['fal', 'repeat'],
}

const iconLibNav = {
	x: ['fal', 'times'],
	hamburger: ['fal', 'bars'],
	circlesV: ['fal', 'ellipsis-v-alt'],
	circlesH: ['fal', 'ellipsis-h-alt'],
	dotsV: ['fal', 'ellipsis-v'],
	dotsH: ['fal', 'ellipsis-h'],
	plus: ['fal', 'plus'],
	login: ['fal', 'sign-in-alt'],
	logout: ['fal', 'sign-out-alt'],
}

const iconLibPeople = {
	user: ['fal', 'user-circle'],
	users: ['fal', 'users'],
	userEdit: ['fal', 'user-edit'],
	userAdd: ['fal', 'user-plus'],
}

const iconLibFinancial = {
	dollar: ['fal', 'dollar-sign'],
	percentage: ['fal', 'percentage'],
	cb: ['fal', 'box-usd'],
	deposit: ['fal', 'envelope-open-dollar'],
	coins: ['fal', 'coins'],
}

const iconLibUtil = {
	dash: ['fal', 'tachometer'],
	mapMarker: ['fal', 'map-marker-alt'],
	film: ['fal', 'film'],
	flag: ['fal', 'flag'],
	camcorder: ['fal', 'camcorder'],
	pages: ['fal', 'file-alt'],
	copy: ['fal', 'copy'],
	clock: ['fal', 'clock'],
	plane: ['fal', 'paper-plane'],
	image: ['fal', 'image'],
	fileUpload: ['fal', 'file-upload'],
	status: ['fal', 'location'],
	folder: ['fal', 'folder-open'],
	utensils: ['fal', 'utensils-alt'],
	alert: ['fal', 'exclamation-triangle'],
	email: ['fal', 'envelope'],
	phone: ['fal', 'phone-alt'],
	lockAlt: ['fal', 'lock-alt'],
	report: ['fal', 'chart-area'],
	edit: ['fal', 'edit'],
	search: ['fal', 'search'],
	calEdit: ['fal', 'calendar-edit'],
	calX: ['fal', 'calendar-times'],
	pdf: ['fal', 'file-pdf'],
	file: ['fal', 'file'],
	connect: ['fal', 'diagram-project'],
}

const iconLibResource = {
	// Single Resources
	parcel: ['fal', 'draw-square'],
	lease: ['fal', 'file-signature'],
	sale: ['fal', 'sack-dollar'],
	acquisition: ['fal', 'file-invoice-dollar'],
	easement: ['fal', 'badge-dollar'],
	agreement: ['fal', 'handshake'],
	doc: ['fal', 'file-import'],
	media: ['fal', 'file-plus'],
	org: ['fal', 'address-card'],
	royalty: ['fal', 'crown'],
	log: ['fal', 'list-alt'],
	contact: ['fal', 'file-user'],
	research: ['fal', 'file-search'],
	exhibit: ['fal', 'file-invoice'],

	// Plural Resources
	parcels: ['fal', 'draw-square'],
	leases: ['fal', 'file-signature'],
	sales: ['fal', 'sack-dollar'],
	acquisitions: ['fal', 'file-invoice-dollar'],
	easements: ['fal', 'badge-dollar'],
	agreements: ['fal', 'handshake'],
	docs: ['fal', 'file-import'],
	medias: ['fal', 'file-plus'],
	orgs: ['fal', 'address-card'],
	royalties: ['fal', 'crown'],
	logs: ['fal', 'list-alt'],
	contacts: ['fal', 'file-user'],

	// Estate Types
	srf: ['fal', 'mountain'],
	min: ['fal', 'gem'],
	ind: ['fal', 'truck-container'],
	oil: ['fal', 'oil-can'],
	geo: ['fal', 'fire-alt'],
}

const iconLibStatus = {
	terminate: ['fal', 'ban'],
	terminated: ['fal', 'ban'],
	cancel: ['fal', 'handshake-slash'],
	cancelled: ['fal', 'handshake-slash'],
	convey: ['fal', 'file-certificate'],
	conveyed: ['fal', 'file-certificate'],
	surfaceConveyed: ['fal', 'file-certificate'],
	surfaceUnowned: ['fal', 'file-minus'],
	closed: ['fal', 'file-certificate'],
	closedEscrow: ['fal', 'file-certificate'],
	foreclosed: ['fal', 'store-alt-slash'],
	foreclose: ['fal', 'store-alt-slash'],
	open: ['fal', 'store-alt'],
	openEscrow: ['fal', 'store-alt'],
	active: ['fal', 'file-check'],
	expire: ['fal', 'calendar-times'],
	expired: ['fal', 'calendar-times'],
	pastExpiration: ['fal', 'exclamation-triangle'],
	available: ['fal', 'tags'],
	noParcels: ['fal', 'exclamation-triangle'],
}

const iconLibMisc = {
	swf: ['fal', 'hammer-war'],
	personSign: ['fal', 'person-sign'],
	handShake: ['fal', 'hands-helping'],
	handShakePro: ['fal', 'handshake-alt'],
	fistPump: ['fal', 'fist-raised'],
	eye: ['fal', 'eye'],
	eyeClosed: ['fal', 'eye-slash'],
	tack: ['fal', 'thumbtack'],
}

export const iconLib = [
	{
		name: 'Alerts',
		group: 'alerts',
		icons: iconLibAlerts,
	},
	{
		name: 'Directional',
		group: 'dir',
		icons: iconLibDirectional,
	},
	{
		name: 'Navigation',
		group: 'nav',
		icons: iconLibNav,
	},
	{
		name: 'People',
		group: 'user',
		icons: iconLibPeople,
	},
	{
		name: 'Financial',
		group: 'financial',
		icons: iconLibFinancial,
	},
	{
		name: 'Utility',
		group: 'utility',
		icons: iconLibUtil,
	},
	{
		name: 'Resource',
		group: 'resource',
		icons: iconLibResource,
	},
	{
		name: 'status',
		group: 'status',
		icons: iconLibStatus,
	},
	{
		name: 'Misc.',
		group: 'misc',
		icons: iconLibMisc,
	},
]

export const iconMap = {
	...iconLibAlerts,
	...iconLibDirectional,
	...iconLibFinancial,
	...iconLibNav,
	...iconLibPeople,
	...iconLibUtil,
	...iconLibMisc,
	...iconLibResource,
	...iconLibStatus,
}

export type IconType = keyof typeof iconMap

const lightIcons = values(light).filter(val => !!get(val, 'icon'))

export const defaultLib: any = [...lightIcons]

library.add(defaultLib)
