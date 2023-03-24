# Rotas da api

## Rotas de autenticação

## Rotas de usuários

## Rotas gerais

> POST /route/

```ts
interface Request {
  start_time?: number; // In minutes from midnight
  end_time?: number; // In minutes from midnight
  origin?: number; // Post id
  destination?: number; // Post id
  day_of_week?: number; // 0-6
  accessibility?: {
    visual?: boolean;
    auditory?: boolean;
    motor?: boolean;
  };
}

interface Response {
  routes: {
    id: number;
    start_time?: number;
    end_time?: number;
    posts: {
      id: number;
      time: number; // In minutes from midnight
    }[]
    day_of_week: number;
    accessibility?: {
      visual: boolean;
      auditory: boolean;
      motor: boolean;
    };
    bus?: {
      id: number;
      name: string;
      capacity: number;
      accessibility: {
        visual: boolean;
        auditory: boolean;
        motor: boolean;
      };
      fuel: number;
    };
  }[];
}
```

> POST /bus/\<id>

```json
```

> POST /post/\<id>

```json
```

> PATCH /update/location

```json
```
