export function generateTimeOptions() {
  const times = []
  for (let hour = 0; hour < 24; hour++) {
    const formattedHour = hour.toString().padStart(2, '0')
    times.push(`${formattedHour}:00`)
    times.push(`${formattedHour}:30`)
  }
  return times
}

export function timeToRFC3339(timeString: string) {
  const now = new Date()
  if (timeString && timeString.trim() != '') {
    const [hours, minutes] = timeString.split(':').map(Number)
    if (hours < 0 || minutes < 0 || hours > 23 || minutes > 59) {
      throw Error('Invalid time, Time should be in range of 00:00 and 23:59')
    }
    now.setHours(hours)
    now.setMinutes(minutes)
  }
  if ((now.getMinutes() != 0 && now.getMinutes() != 30 ) || timeString == '') {
    if (60 - now.getMinutes() < now.getMinutes()) {
      now.setHours(now.getHours() + 1)
      now.setMinutes(0)
    } else {
      now.setMinutes(30)
    }
  }
  now.setSeconds(0)
  now.setMilliseconds(0)
  return formatToRFC3339(now)
}

function formatToRFC3339(date: Date) {
  const pad = (num: number) => num.toString().padStart(2, '0')

  const year = date.getFullYear()
  const month = pad(date.getMonth() + 1) // getMonth() is 0-indexed
  const day = pad(date.getDate())
  const hours = pad(date.getHours())
  const minutes = pad(date.getMinutes())
  const seconds = pad(date.getSeconds())

  // Use +07:00 for GMT+7
  return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}+07:00`
}